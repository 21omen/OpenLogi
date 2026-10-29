//! The mice whose extra buttons are remapped in HID++ Host mode, and what
//! their `0x8110` button-spy bits mean.
//!
//! A gaming mouse without `0x1b04` cannot divert its extra buttons. Instead
//! the session switches the mouse to Host mode (`0x8100`), zeroes those
//! buttons' slots in the `0x8110` HID mapping so the OS stops seeing them, and
//! reads presses from the button-spy mask. Host mode also pauses the onboard
//! profiles, so the firmware's own DPI buttons stop working while it is on.
//!
//! Each supported model is one [`SpyModel`] row in [`SPY_MODELS`], keyed by
//! its HID++ model key ([`DeviceModelInfo::model_key`]). The row is the single
//! owner of everything model-specific: which extra buttons exist, which OS
//! side buttons the spy takes over when remapped, and the bit of the spy mask
//! (= the slot of the mapping) each one reports on. The capture session, the
//! agent's plan and the GUI all read it from here.
//!
//! # Adding a model
//!
//! Only from a live dump, never by copying a cousin's row: two variants of one
//! mouse can report different bits.
//!
//! 1. Quit Options+ / G HUB so OpenLogi owns the receiver.
//! 2. Run `openlogi diag mouse-buttons --watch`, press each button, and note
//!    bit → button.
//! 3. Add a row. List the side buttons (Back/Forward) in
//!    `spy_owned_os_buttons` only when the mouse has no `0x1b04`: otherwise
//!    that feature already diverts them.
//!
//! The extra buttons stay out of [`ButtonId::ALL`]: that list seeds every
//! mouse's defaults and popover.
//!
//! [`DeviceModelInfo::model_key`]: crate::device::DeviceModelInfo::model_key

use std::collections::BTreeMap;

use super::button::ButtonId;
use super::defaults::default_binding;
use super::value::Binding;

/// One bit of a model's `0x8110` spy mask and the button it reports.
///
/// The bit index is also the button's slot in the `0x8110` HID mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpyBit {
    bit: u8,
    button: ButtonId,
}

impl SpyBit {
    /// The button reported on `bit` of the 16-bit spy mask.
    ///
    /// # Panics
    ///
    /// When `bit` is not below 16 — at compile time, since the model table is
    /// a `const`.
    #[must_use]
    pub const fn new(bit: u8, button: ButtonId) -> Self {
        assert!(bit < 16, "the 0x8110 spy mask has 16 bits");
        Self { bit, button }
    }

    /// The button this bit reports.
    #[must_use]
    pub const fn button(self) -> ButtonId {
        self.button
    }

    const fn mask(self) -> u16 {
        1 << self.bit
    }
}

/// A mouse whose extra buttons are remapped in Host mode through `0x8110`.
#[derive(Debug, PartialEq, Eq)]
pub struct SpyModel {
    model_key: &'static str,
    extra_buttons: &'static [ButtonId],
    spy_owned_os_buttons: &'static [ButtonId],
    bits: &'static [SpyBit],
}

/// G6–G9 of the G502 X Plus.
const G502_X_PLUS_EXTRAS: [ButtonId; 4] = [
    ButtonId::DpiShift,
    ButtonId::DpiUp,
    ButtonId::DpiDown,
    ButtonId::ProfileCycle,
];

/// The spy bits of the G502 X Plus reporting extended model id 0, from the
/// live `diag mouse-buttons --watch` of the original Host-mode work.
const G502_X_PLUS_BITS: [SpyBit; 6] = [
    SpyBit::new(3, ButtonId::Back),
    SpyBit::new(4, ButtonId::DpiShift),
    SpyBit::new(5, ButtonId::Forward),
    SpyBit::new(8, ButtonId::ProfileCycle),
    SpyBit::new(9, ButtonId::DpiUp),
    SpyBit::new(10, ButtonId::DpiDown),
];

/// The spy bits of the G502 X Plus reporting extended model id 1 (wpid 4099
/// on a LIGHTSPEED receiver `046d:c547`, model ids `4099 c095 0000`), from
/// its own live `diag mouse-buttons --watch` (11 slots, mapping `01..0b`).
/// They match the extended-id-0 dump, which this row does not rely on.
const G502_X_PLUS_EXT1_BITS: [SpyBit; 6] = [
    SpyBit::new(3, ButtonId::Back),
    SpyBit::new(4, ButtonId::DpiShift),
    SpyBit::new(5, ButtonId::Forward),
    SpyBit::new(8, ButtonId::ProfileCycle),
    SpyBit::new(9, ButtonId::DpiUp),
    SpyBit::new(10, ButtonId::DpiDown),
];

/// Every model with a recorded spy-bit table.
const SPY_MODELS: [SpyModel; 2] = [
    SpyModel {
        // G502 X Plus, wpid 4099, extended model id 0.
        model_key: "04099",
        extra_buttons: &G502_X_PLUS_EXTRAS,
        // No `0x1b04`: a remapped G4/G5 must leave the HID mapping, or the OS
        // keeps seeing hardware Back/Forward.
        spy_owned_os_buttons: &[ButtonId::Back, ButtonId::Forward],
        bits: &G502_X_PLUS_BITS,
    },
    SpyModel {
        // G502 X Plus, wpid 4099, extended model id 1.
        model_key: "14099",
        extra_buttons: &G502_X_PLUS_EXTRAS,
        // No `0x1b04` either.
        spy_owned_os_buttons: &[ButtonId::Back, ButtonId::Forward],
        bits: &G502_X_PLUS_EXT1_BITS,
    },
];

impl SpyModel {
    /// The row for the HID++ model key `model_key`, if that model has one.
    #[must_use]
    pub fn for_model_key(model_key: &str) -> Option<&'static Self> {
        SPY_MODELS.iter().find(|model| model.model_key == model_key)
    }

    /// The HID++ model key this row belongs to.
    #[must_use]
    pub fn model_key(&self) -> &'static str {
        self.model_key
    }

    /// The model's extra buttons, which no other mouse has. Kept out of
    /// [`ButtonId::ALL`].
    #[must_use]
    pub fn extra_buttons(&self) -> &'static [ButtonId] {
        self.extra_buttons
    }

    /// The buttons a capture session takes over in Host mode under
    /// `bindings`, in table order; empty when it needs no Host mode.
    ///
    /// One configured extra takes the whole extra cluster, since Host mode
    /// stops the firmware's own function for all of them. A side button
    /// joins only while it leaves its native function, so an unbound G4/G5
    /// stays firmware-native.
    #[must_use]
    pub fn armed_buttons(&self, bindings: &BTreeMap<ButtonId, Binding>) -> Vec<ButtonId> {
        let extras_configured = self.extra_buttons.iter().any(|button| {
            bindings
                .get(button)
                .is_some_and(Binding::has_configured_action)
        });
        let side_remapped = |button: &ButtonId| {
            bindings
                .get(button)
                .is_some_and(|binding| leaves_native(*button, binding))
        };
        self.bits
            .iter()
            .map(|bit| bit.button)
            .filter(|button| {
                if self.extra_buttons.contains(button) {
                    extras_configured
                } else {
                    self.spy_owned_os_buttons.contains(button) && side_remapped(button)
                }
            })
            .collect()
    }

    /// The press (`true`) and release (`false`) edges of the `armed` buttons
    /// between two spy masks, in table order.
    pub fn edges<'a>(
        &'a self,
        previous: u16,
        next: u16,
        armed: &'a [ButtonId],
    ) -> impl Iterator<Item = (ButtonId, bool)> + 'a {
        let changed = previous ^ next;
        self.bits
            .iter()
            .filter(move |bit| changed & bit.mask() != 0 && armed.contains(&bit.button))
            .map(move |bit| (bit.button, next & bit.mask() != 0))
    }

    /// `mapping` with the slots of the `armed` buttons zeroed, which removes
    /// them from the mouse's standard HID reports while it is in Host mode.
    #[must_use]
    pub fn suppressed_mapping(&self, mapping: &[u8], armed: &[ButtonId]) -> Vec<u8> {
        let mut suppressed = mapping.to_vec();
        for bit in self.bits.iter().filter(|bit| armed.contains(&bit.button)) {
            if let Some(slot) = suppressed.get_mut(usize::from(bit.bit)) {
                *slot = 0;
            }
        }
        suppressed
    }
}

/// Whether a side button bound to `binding` does anything but its native
/// click.
fn leaves_native(button: ButtonId, binding: &Binding) -> bool {
    match binding {
        Binding::Single(action) => *action != default_binding(button),
        Binding::Gesture(_) | Binding::LongPress(_) | Binding::WheelHold(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding::{Action, LongPressBinding};

    fn g502() -> &'static SpyModel {
        SpyModel::for_model_key("04099").expect("the G502 X Plus has a row")
    }

    #[test]
    fn only_recorded_models_have_a_row() {
        assert_eq!(g502().extra_buttons(), G502_X_PLUS_EXTRAS.as_slice());
        let ext1 = SpyModel::for_model_key("14099").expect("the extended-id-1 G502 has a row");
        assert_eq!(ext1.model_key(), "14099");
        assert_eq!(ext1.extra_buttons(), G502_X_PLUS_EXTRAS.as_slice());
        assert_eq!(SpyModel::for_model_key("2b042"), None);
        assert_eq!(SpyModel::for_model_key("0409f"), None);
    }

    #[test]
    fn every_row_is_consistent() {
        for model in &SPY_MODELS {
            for button in model.extra_buttons {
                assert!(!ButtonId::ALL.contains(button), "{button:?} in ALL");
                assert_eq!(default_binding(*button), Action::None);
                assert!(!button.is_os_hook_button());
            }
            for button in model.extra_buttons.iter().chain(model.spy_owned_os_buttons) {
                assert_eq!(
                    model
                        .bits
                        .iter()
                        .filter(|bit| bit.button == *button)
                        .count(),
                    1,
                    "{button:?} needs exactly one bit on {}",
                    model.model_key
                );
            }
            for (index, bit) in model.bits.iter().enumerate() {
                assert!(
                    model.bits[..index].iter().all(|other| other.bit != bit.bit),
                    "bit {} listed twice on {}",
                    bit.bit,
                    model.model_key
                );
            }
        }
    }

    #[test]
    fn unbound_model_needs_no_host_mode() {
        assert!(g502().armed_buttons(&BTreeMap::new()).is_empty());
        let unbound = BTreeMap::from([
            (ButtonId::DpiUp, Binding::Single(Action::None)),
            (ButtonId::Back, Binding::Single(Action::MouseBack)),
        ]);
        assert!(g502().armed_buttons(&unbound).is_empty());
    }

    #[test]
    fn one_configured_extra_arms_the_whole_cluster() {
        let bindings = BTreeMap::from([(ButtonId::DpiUp, Binding::Single(Action::Copy))]);
        assert_eq!(
            g502().armed_buttons(&bindings),
            [
                ButtonId::DpiShift,
                ButtonId::ProfileCycle,
                ButtonId::DpiUp,
                ButtonId::DpiDown,
            ]
        );
    }

    #[test]
    fn click_less_long_press_arms_the_cluster() {
        let bindings = BTreeMap::from([(
            ButtonId::DpiShift,
            Binding::LongPress(LongPressBinding::new(Action::None, Action::MissionControl)),
        )]);
        assert_eq!(g502().armed_buttons(&bindings).len(), 4);
    }

    #[test]
    fn a_remapped_side_button_joins_alone() {
        let bindings = BTreeMap::from([
            (ButtonId::Back, Binding::Single(Action::PreviousDesktop)),
            (ButtonId::Forward, Binding::Single(Action::MouseForward)),
        ]);
        assert_eq!(g502().armed_buttons(&bindings), [ButtonId::Back]);
    }

    /// The extended-id-1 row against its own dump: G8 is bit 9, and arming
    /// the extras zeroes slots 4, 8, 9 and 10 of mapping `01..0b`.
    #[test]
    fn the_extended_id_1_row_follows_its_own_dump() {
        let ext1 = SpyModel::for_model_key("14099").expect("row");
        let armed = ext1.armed_buttons(&BTreeMap::from([(
            ButtonId::DpiUp,
            Binding::Single(Action::Copy),
        )]));
        assert_eq!(armed.len(), 4);
        let down: Vec<_> = ext1.edges(0, 1 << 9, &armed).collect();
        assert_eq!(down, [(ButtonId::DpiUp, true)]);
        let mapping: Vec<u8> = (1..=11).collect();
        assert_eq!(
            ext1.suppressed_mapping(&mapping, &armed),
            [1, 2, 3, 4, 0, 6, 7, 8, 0, 0, 0]
        );
    }

    #[test]
    fn edges_follow_armed_bits_only() {
        let armed = [ButtonId::DpiUp];
        let down: Vec<_> = g502().edges(0, 0x0200, &armed).collect();
        assert_eq!(down, [(ButtonId::DpiUp, true)]);
        let up: Vec<_> = g502().edges(0x0200, 0, &armed).collect();
        assert_eq!(up, [(ButtonId::DpiUp, false)]);
        // G4 (bit 3) and G7 (bit 10) are not armed here.
        assert_eq!(g502().edges(0, 0x0408, &armed).count(), 0);
    }

    #[test]
    fn edges_report_simultaneous_changes_in_table_order() {
        let armed = [ButtonId::Back, ButtonId::DpiShift];
        let edges: Vec<_> = g502().edges(0x0010, 0x0008, &armed).collect();
        assert_eq!(edges, [(ButtonId::Back, true), (ButtonId::DpiShift, false)]);
    }

    #[test]
    fn suppression_zeroes_only_armed_slots() {
        let mapping: Vec<u8> = (1..=11).collect();
        let extras = g502().suppressed_mapping(&mapping, &G502_X_PLUS_EXTRAS);
        assert_eq!(extras, [1, 2, 3, 4, 0, 6, 7, 8, 0, 0, 0]);
        let back = g502().suppressed_mapping(&mapping, &[ButtonId::Back]);
        assert_eq!(back, [1, 2, 3, 0, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn suppression_ignores_slots_beyond_a_short_mapping() {
        let short = [1, 2, 3, 4, 5];
        assert_eq!(
            g502().suppressed_mapping(&short, &G502_X_PLUS_EXTRAS),
            [1, 2, 3, 4, 0]
        );
    }
}
