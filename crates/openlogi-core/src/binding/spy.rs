//! The mice whose buttons are remapped in HID++ Host mode, and what their
//! `0x8110` button-spy bits mean.
//!
//! A gaming mouse without `0x1b04` cannot divert its buttons. Instead the
//! session switches the mouse to Host mode (`0x8100`), zeroes the slots of the
//! buttons it takes over in the `0x8110` HID mapping so the OS stops seeing
//! them, and reads their presses from the button-spy mask.
//!
//! Host mode changes what every other button sends, too: onboard profiles
//! pause, and slot *n* reports plain mouse button *n + 1*. For the side
//! buttons that is still their job (G4/G5 report buttons 4 and 6, Back and
//! Forward). For the rest it is not: the G502's DPI and profile buttons lose
//! their firmware function, its sniper button G6 would report button 5 — which
//! the OS takes for Forward — and its wheel tilts stop scrolling and report
//! buttons 7 and 8. So once a mouse is in Host mode, the session takes over
//! every button of its table except those that keep their job, and a side
//! button only while it is remapped.
//!
//! Each supported model is one [`SpyModel`] row in [`SPY_MODELS`], keyed by
//! its HID++ model key ([`DeviceModelInfo::model_key`]). The row is the single
//! owner of everything model-specific: which button each bit of the spy mask
//! (= each slot of the mapping) reports, and which of them keep their job in
//! Host mode. The capture session, the agent's plan and the GUI all read it
//! from here.
//!
//! # Adding a model
//!
//! Only from a live dump, never by copying a cousin's row: two variants of one
//! mouse can report different bits.
//!
//! 1. Quit Options+ / G HUB so OpenLogi owns the receiver.
//! 2. Run `openlogi diag mouse-buttons --watch`, press each button, and note
//!    bit → button. Leave a button out rather than guess its bit: Host mode
//!    then leaves it as the mapping says.
//! 3. Add a row. List in `host_native` the buttons whose Host-mode mapping code
//!    still does their own job.
//!
//! A model's extra buttons (the G502's G6–G9) stay out of [`ButtonId::ALL`]:
//! that list seeds every mouse's defaults and popover.
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

/// A mouse whose buttons are remapped in Host mode through `0x8110`.
#[derive(Debug, PartialEq, Eq)]
pub struct SpyModel {
    model_key: &'static str,
    bits: &'static [SpyBit],
    /// The buttons of `bits` whose Host-mode mapping code still does their
    /// own job: taken over only while remapped.
    host_native: &'static [ButtonId],
}

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
/// Bits 3–5 and 8–10 match the extended-id-0 dump, which this row does not
/// rely on; that dump did not record the wheel tilts.
const G502_X_PLUS_EXT1_BITS: [SpyBit; 8] = [
    SpyBit::new(3, ButtonId::Back),
    SpyBit::new(4, ButtonId::DpiShift),
    SpyBit::new(5, ButtonId::Forward),
    SpyBit::new(6, ButtonId::WheelTiltLeft),
    SpyBit::new(7, ButtonId::WheelTiltRight),
    SpyBit::new(8, ButtonId::ProfileCycle),
    SpyBit::new(9, ButtonId::DpiUp),
    SpyBit::new(10, ButtonId::DpiDown),
];

/// Every model with a recorded spy-bit table.
const SPY_MODELS: [SpyModel; 2] = [
    SpyModel {
        // G502 X Plus, wpid 4099, extended model id 0.
        model_key: "04099",
        bits: &G502_X_PLUS_BITS,
        host_native: &[ButtonId::Back, ButtonId::Forward],
    },
    SpyModel {
        // G502 X Plus, wpid 4099, extended model id 1.
        model_key: "14099",
        bits: &G502_X_PLUS_EXT1_BITS,
        host_native: &[ButtonId::Back, ButtonId::Forward],
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

    /// Every button the spy can report on this model, in table order.
    pub fn buttons(&self) -> impl Iterator<Item = ButtonId> + '_ {
        self.bits.iter().map(|bit| bit.button)
    }

    /// The buttons a capture session takes over in Host mode under
    /// `bindings`, in table order; empty when it needs no Host mode.
    ///
    /// Any button of the table that leaves its default puts the mouse in
    /// Host mode. Every button whose job Host mode breaks is then taken over,
    /// bound or not, so none reports a mapping code in its place; a button
    /// that keeps its job joins only while it is remapped, so an unbound
    /// G4/G5 stays firmware-native.
    #[must_use]
    pub fn armed_buttons(&self, bindings: &BTreeMap<ButtonId, Binding>) -> Vec<ButtonId> {
        let remapped = |button: ButtonId| {
            bindings
                .get(&button)
                .is_some_and(|binding| leaves_default(button, binding))
        };
        if !self.buttons().any(remapped) {
            return Vec::new();
        }
        self.buttons()
            .filter(|button| !self.host_native.contains(button) || remapped(*button))
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

/// Whether `button` bound to `binding` does anything but its default.
fn leaves_default(button: ButtonId, binding: &Binding) -> bool {
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

    fn ext1() -> &'static SpyModel {
        SpyModel::for_model_key("14099").expect("the extended-id-1 G502 has a row")
    }

    const EXTRAS: [ButtonId; 4] = [
        ButtonId::DpiShift,
        ButtonId::ProfileCycle,
        ButtonId::DpiUp,
        ButtonId::DpiDown,
    ];

    fn single(button: ButtonId, action: Action) -> BTreeMap<ButtonId, Binding> {
        BTreeMap::from([(button, Binding::Single(action))])
    }

    #[test]
    fn only_recorded_models_have_a_row() {
        assert_eq!(g502().model_key(), "04099");
        assert_eq!(ext1().model_key(), "14099");
        assert_eq!(SpyModel::for_model_key("2b042"), None);
        assert_eq!(SpyModel::for_model_key("0409f"), None);
    }

    #[test]
    fn every_row_is_consistent() {
        for model in &SPY_MODELS {
            for (index, bit) in model.bits.iter().enumerate() {
                assert!(
                    model.bits[..index]
                        .iter()
                        .all(|other| other.bit != bit.bit && other.button != bit.button),
                    "bit {} or {:?} listed twice on {}",
                    bit.bit,
                    bit.button,
                    model.model_key
                );
                if !ButtonId::ALL.contains(&bit.button) {
                    // An extra seeds no default, so an unbound one never
                    // enters Host mode.
                    assert_eq!(default_binding(bit.button), Action::None);
                    assert!(!bit.button.is_os_hook_button());
                }
            }
            for button in model.host_native {
                assert!(
                    model.buttons().any(|listed| listed == *button),
                    "{button:?} keeps its job but has no bit on {}",
                    model.model_key
                );
            }
        }
    }

    #[test]
    fn unbound_model_needs_no_host_mode() {
        assert!(ext1().armed_buttons(&BTreeMap::new()).is_empty());
        let defaults = BTreeMap::from([
            (ButtonId::DpiUp, Binding::Single(Action::None)),
            (ButtonId::Back, Binding::Single(Action::MouseBack)),
            (
                ButtonId::WheelTiltLeft,
                Binding::Single(Action::HorizontalScrollLeft),
            ),
        ]);
        assert!(ext1().armed_buttons(&defaults).is_empty());
    }

    /// One bound extra puts the mouse in Host mode, which takes over every
    /// button whose job it breaks — the other extras and the wheel tilts —
    /// but leaves the unbound side buttons native.
    #[test]
    fn host_mode_takes_every_button_it_breaks() {
        let armed = ext1().armed_buttons(&single(ButtonId::DpiUp, Action::Copy));
        assert_eq!(
            armed,
            [
                ButtonId::DpiShift,
                ButtonId::WheelTiltLeft,
                ButtonId::WheelTiltRight,
                ButtonId::ProfileCycle,
                ButtonId::DpiUp,
                ButtonId::DpiDown,
            ]
        );
    }

    #[test]
    fn a_remapped_tilt_alone_enters_host_mode() {
        let armed = ext1().armed_buttons(&single(ButtonId::WheelTiltRight, Action::NextDesktop));
        assert!(armed.contains(&ButtonId::WheelTiltRight));
        assert!(armed.contains(&ButtonId::WheelTiltLeft));
        assert!(EXTRAS.iter().all(|extra| armed.contains(extra)));
        assert!(!armed.contains(&ButtonId::Back));
    }

    /// A remapped side button enters Host mode too, and then G6 must be
    /// taken over: its mapping code would otherwise reach the OS as Forward.
    #[test]
    fn a_remapped_side_button_takes_the_broken_buttons_along() {
        let bindings = BTreeMap::from([
            (ButtonId::Back, Binding::Single(Action::PreviousDesktop)),
            (ButtonId::Forward, Binding::Single(Action::MouseForward)),
        ]);
        let armed = g502().armed_buttons(&bindings);
        assert_eq!(
            armed,
            [
                ButtonId::Back,
                ButtonId::DpiShift,
                ButtonId::ProfileCycle,
                ButtonId::DpiUp,
                ButtonId::DpiDown,
            ]
        );
    }

    #[test]
    fn click_less_long_press_enters_host_mode() {
        let bindings = BTreeMap::from([(
            ButtonId::DpiShift,
            Binding::LongPress(LongPressBinding::new(Action::None, Action::MissionControl)),
        )]);
        assert_eq!(g502().armed_buttons(&bindings), EXTRAS);
    }

    /// The extended-id-1 row against its own dump: G8 is bit 9, the tilts
    /// bits 6 and 7, and Host mode zeroes slots 4 and 6–10 of mapping
    /// `01..0b`.
    #[test]
    fn the_extended_id_1_row_follows_its_own_dump() {
        let armed = ext1().armed_buttons(&single(ButtonId::DpiUp, Action::Copy));
        let down: Vec<_> = ext1().edges(0, 1 << 9, &armed).collect();
        assert_eq!(down, [(ButtonId::DpiUp, true)]);
        let tilt: Vec<_> = ext1().edges(0, 1 << 6, &armed).collect();
        assert_eq!(tilt, [(ButtonId::WheelTiltLeft, true)]);
        let mapping: Vec<u8> = (1..=11).collect();
        assert_eq!(
            ext1().suppressed_mapping(&mapping, &armed),
            [1, 2, 3, 4, 0, 6, 0, 0, 0, 0, 0]
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
        let extras = g502().suppressed_mapping(&mapping, &EXTRAS);
        assert_eq!(extras, [1, 2, 3, 4, 0, 6, 7, 8, 0, 0, 0]);
        let back = g502().suppressed_mapping(&mapping, &[ButtonId::Back]);
        assert_eq!(back, [1, 2, 3, 0, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn suppression_ignores_slots_beyond_a_short_mapping() {
        let short = [1, 2, 3, 4, 5];
        assert_eq!(g502().suppressed_mapping(&short, &EXTRAS), [1, 2, 3, 4, 0]);
    }
}
