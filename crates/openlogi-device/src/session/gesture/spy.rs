//! Host-mode capture of a [`SpyModel`]'s buttons over `0x8110`.
//!
//! Arming switches the mouse to Host mode (`0x8100`), zeroes the armed
//! buttons' slots in the `0x8110` HID mapping so the OS stops seeing them,
//! and starts the button spy, whose mask reports every button the session
//! then reads. Teardown stops the spy, writes the mapping back and returns the
//! mouse to its onboard profiles.
//!
//! Both features are reverse-engineered (see their modules in `hidpp`); the
//! bit each button reports on is the [`SpyModel`]'s, recorded from a live
//! dump.

use std::sync::Arc;

use hidpp::channel::HidppChannel;
use hidpp::device::Device;
use hidpp::feature::CreatableFeature as _;
use hidpp::feature::mouse_button_filter::MouseButtonFilterFeature;
use hidpp::feature::onboard_profiles::{OnboardProfilesFeature, OnboardProfilesMode};
use hidpp::protocol::v20::{self, Hidpp20Error};
use openlogi_core::binding::{ButtonId, SpyModel};
use tracing::warn;

use super::arm::ArmedControls;
use crate::session::capture_restore::{CaptureError, restore_result};

/// The buttons one capture session takes over in Host mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpyCapture {
    /// The model whose spy bits the buttons report on.
    pub model: &'static SpyModel,
    /// What [`SpyModel::armed_buttons`] chose; never empty.
    pub buttons: Vec<ButtonId>,
}

/// The Host-mode capture of an armed session.
pub(super) struct ArmedSpy {
    filter: MouseButtonFilterFeature,
    profiles: OnboardProfilesFeature,
    restore: SpyRestore,
    model: &'static SpyModel,
    buttons: Vec<ButtonId>,
    /// The mapping with the armed buttons' slots zeroed.
    suppressed: Vec<u8>,
}

impl ArmedSpy {
    pub(super) fn feature_index(&self) -> u8 {
        self.restore.filter_index
    }

    pub(super) fn model(&self) -> &'static SpyModel {
        self.model
    }

    pub(super) fn buttons(&self) -> &[ButtonId] {
        &self.buttons
    }

    pub(super) fn restore(&self) -> SpyRestore {
        self.restore.clone()
    }

    /// Enter Host mode, suppress the armed buttons and start the spy.
    ///
    /// Also the wake re-arm: a power cycle returns the mouse to its onboard
    /// profiles and its own mapping, and stops the spy.
    pub(super) async fn apply(&self) -> Result<(), Hidpp20Error> {
        self.profiles.set_mode(OnboardProfilesMode::Host).await?;
        self.filter
            .set_mouse_button_mapping(&self.suppressed)
            .await?;
        self.filter.start_mouse_button_spy().await
    }
}

/// What teardown writes to hand a Host-mode capture back.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SpyRestore {
    filter_index: u8,
    profiles_index: u8,
    /// The mapping read before arming.
    mapping: Vec<u8>,
}

impl SpyRestore {
    /// Stop the spy, write the mapping back and return the mouse to its
    /// onboard profiles.
    ///
    /// The mode always goes back to onboard, even when arming found Host
    /// mode already on: that is a session that never tore down, and handing
    /// it back would leave the buttons it suppressed dead. Onboard mode
    /// ignores the mapping, so a mapping read from such a leftover is
    /// harmless to write back.
    pub(crate) async fn restore_on(&self, channel: &Arc<HidppChannel>, device_index: u8) -> bool {
        let filter =
            MouseButtonFilterFeature::new(Arc::clone(channel), device_index, self.filter_index);
        let profiles =
            OnboardProfilesFeature::new(Arc::clone(channel), device_index, self.profiles_index);
        // A refused stop means the spy is not running, which is what teardown
        // wants: rollback can owe this restore before the spy ever started.
        // Only a transport error leaves the spy's state unknown.
        let mut restored = match filter.stop_mouse_button_spy().await {
            Err(error @ Hidpp20Error::Channel(_)) => restore_result(Err(error), "button spy"),
            Ok(()) | Err(_) => true,
        };
        restored &= restore_result(
            filter.set_mouse_button_mapping(&self.mapping).await,
            "button mapping",
        );
        restored &= restore_result(
            profiles.set_mode(OnboardProfilesMode::Onboard).await,
            "onboard profiles",
        );
        restored
    }
}

/// Take over `spy`'s buttons in Host mode.
///
/// A mouse missing either feature, or refusing to be read, stays native and
/// the rest of the session arms anyway. A refused write hands the mouse back
/// the same way. A transport failure fails arming, which rolls everything
/// back.
pub(super) async fn arm_spy(
    device: &Device,
    chan: &Arc<HidppChannel>,
    slot: u8,
    spy: &SpyCapture,
    armed: &mut ArmedControls,
) -> Result<(), CaptureError> {
    let root = device.root();
    let (Some(filter), Some(profiles)) = (
        root.get_feature(MouseButtonFilterFeature::ID).await?,
        root.get_feature(OnboardProfilesFeature::ID).await?,
    ) else {
        warn!(
            model = spy.model.model_key(),
            "0x8110 or 0x8100 missing — extra buttons left native"
        );
        return Ok(());
    };
    let filter_feature = MouseButtonFilterFeature::new(Arc::clone(chan), slot, filter.index);
    let mapping = match filter_feature.get_mouse_button_mapping().await {
        Ok(mapping) => mapping,
        Err(error @ Hidpp20Error::Channel(_)) => return Err(error.into()),
        Err(error) => {
            warn!(
                ?error,
                "button mapping unreadable — extra buttons left native"
            );
            return Ok(());
        }
    };
    let suppressed = spy.model.suppressed_mapping(&mapping, &spy.buttons);
    // Recorded before the first write, which a transport error leaves
    // uncertain: rollback then owes the restore.
    let armed_spy = armed.spy.insert(ArmedSpy {
        filter: filter_feature,
        profiles: OnboardProfilesFeature::new(Arc::clone(chan), slot, profiles.index),
        restore: SpyRestore {
            filter_index: filter.index,
            profiles_index: profiles.index,
            mapping,
        },
        model: spy.model,
        buttons: spy.buttons.clone(),
        suppressed,
    });
    match armed_spy.apply().await {
        Ok(()) => Ok(()),
        Err(error @ Hidpp20Error::Channel(_)) => Err(error.into()),
        Err(error) => {
            warn!(?error, "Host mode refused — handing the extra buttons back");
            if !armed_spy.restore.restore_on(chan, slot).await {
                // Still owed: rollback retries it.
                return Err(error.into());
            }
            armed.spy = None;
            Ok(())
        }
    }
}

/// The spy mask of an unsolicited `0x8110` report for this session's device,
/// or `None` for anything else.
pub(super) fn decode_event(msg: &v20::Message, device_index: u8, feature_index: u8) -> Option<u16> {
    let header = msg.header();
    if header.device_index != device_index
        || header.feature_index != feature_index
        || header.software_id.to_lo() != 0
        || header.function_id.to_lo() != 0
    {
        return None;
    }
    let payload = msg.extend_payload();
    Some(u16::from_be_bytes([payload[0], payload[1]]))
}
