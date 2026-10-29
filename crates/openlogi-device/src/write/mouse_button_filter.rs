//! HID++ `0x8110 MouseButtonFilter` diagnostics: the button count, the HID
//! mapping, and a watch of the button-spy mask.
//!
//! Read and spy only: nothing here enters Host mode (`0x8100`) or writes the
//! mapping, so an interrupted diagnostic cannot leave a mouse with suppressed
//! buttons. A spy left running by a dropped watch only streams masks until the
//! next power cycle; it changes no button behavior.

use std::future::Future;
use std::sync::Arc;

use hidpp::{
    channel::HidppChannel,
    device::Device,
    feature::{
        CreatableFeature, EmittingFeature,
        mouse_button_filter::{MouseButtonFilterEvent, MouseButtonFilterFeature},
    },
    protocol::v20::Hidpp20Error,
};

use crate::backend::HidBackend;
use crate::channel::route::DeviceRoute;
use crate::write::{HidppOperation, WriteError, classify_hidpp_error, open_feature, with_route};

/// Snapshot of the device's `0x8110` button count and HID mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseButtonFilterDump {
    /// Spy-button slots the device reports.
    pub button_count: u8,
    /// One mapping byte per slot: `0` disabled, `1..=16` enabled HID codes.
    pub mapping: Vec<u8>,
}

/// Read the `0x8110` button count and current mapping of the device on
/// `route`.
pub async fn dump_mouse_button_filter(
    backend: &dyn HidBackend,
    route: &DeviceRoute,
) -> Result<MouseButtonFilterDump, WriteError> {
    let index = route.device_index();
    with_route(backend, route, move |channel| async move {
        let feature = open_mouse_button_filter(&channel, index).await?;
        // The mapping read fetches the count first to slice its payload; its
        // length is the count, without a third round-trip.
        let mapping = feature
            .get_mouse_button_mapping()
            .await
            .map_err(dump_error)?;
        let button_count = u8::try_from(mapping.len())
            .map_err(|_| dump_error(Hidpp20Error::UnsupportedResponse))?;
        Ok(MouseButtonFilterDump {
            button_count,
            mapping,
        })
    })
    .await
}

/// Start the button spy on `route`, deliver each mask to `on_event`, and stop
/// the spy once `cancel` resolves or the event stream ends.
///
/// The spy is stopped only when this future runs to completion; `cancel`
/// (Ctrl-C in the CLI) is the way to end a watch.
pub async fn watch_mouse_button_filter<F, C>(
    backend: &dyn HidBackend,
    route: &DeviceRoute,
    mut on_event: F,
    cancel: C,
) -> Result<(), WriteError>
where
    F: FnMut(u16),
    C: Future<Output = ()>,
{
    let index = route.device_index();
    with_route(backend, route, move |channel| async move {
        let feature = open_mouse_button_filter(&channel, index).await?;
        // Listen before starting the spy so the first mask is not lost.
        let events = feature.listen();
        feature.start_mouse_button_spy().await.map_err(dump_error)?;
        tokio::pin!(cancel);
        loop {
            tokio::select! {
                event = events.recv() => match event {
                    Ok(MouseButtonFilterEvent::Buttons { mask }) => on_event(mask),
                    Ok(_) => {}
                    Err(_) => break,
                },
                () = &mut cancel => break,
            }
        }
        feature.stop_mouse_button_spy().await.map_err(dump_error)
    })
    .await
}

async fn open_mouse_button_filter(
    channel: &Arc<HidppChannel>,
    index: u8,
) -> Result<Arc<MouseButtonFilterFeature>, WriteError> {
    let mut device = Device::new(Arc::clone(channel), index)
        .await
        .map_err(|_| WriteError::DeviceUnreachable { index })?;
    open_feature::<MouseButtonFilterFeature>(&mut device).await
}

fn dump_error(error: Hidpp20Error) -> WriteError {
    classify_hidpp_error(
        error,
        HidppOperation::DumpFeatures,
        MouseButtonFilterFeature::ID,
    )
}
