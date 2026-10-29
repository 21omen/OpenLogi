//! Mouse hotspot geometry. Bounds are authored in model-local pixels (the
//! SVG canvas is 420×560 — see [`MOUSE_MODEL_SIZE`]) and
//! stored as plain `f32` tuples so this module stays purely data and doesn't
//! drag in `gpui` types.

use openlogi_core::binding::{ButtonId, SpyModel};

/// One visual target in the mouse diagram.
///
/// Most targets correspond to one physical button. Thumb-wheel rotation is a
/// single visual target backed by two directional bindings, so it has its own
/// identity rather than pretending to be either direction or the wheel click.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::From)]
pub(crate) enum MouseControlId {
    Button(ButtonId),
    ThumbwheelRotation,
}

impl MouseControlId {
    /// Return the physical button when this target represents one.
    #[must_use]
    pub(crate) const fn button(self) -> Option<ButtonId> {
        match self {
            Self::Button(button) => Some(button),
            Self::ThumbwheelRotation => None,
        }
    }

    /// Collapse either live thumb-wheel direction into the one diagram target.
    #[must_use]
    pub(crate) const fn from_active_button(button: ButtonId) -> Self {
        match button {
            ButtonId::ThumbwheelScrollUp | ButtonId::ThumbwheelScrollDown => {
                Self::ThumbwheelRotation
            }
            _ => Self::Button(button),
        }
    }

    #[must_use]
    pub(crate) fn translation_key(self) -> &'static str {
        match self {
            Self::Button(button) => button.translation_key(),
            Self::ThumbwheelRotation => "pointer.thumb_wheel",
        }
    }
}

/// The size of the mouse model canvas. Hotspot coords are relative to this.
pub const MOUSE_MODEL_SIZE: (f32, f32) = (420., 560.);

/// Hotspot rectangle in mouse-model-local coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hotspot {
    pub(crate) id: MouseControlId,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Hotspot {
    /// Returns the center point — convenient for leader lines.
    #[inline]
    #[must_use]
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w * 0.5, self.y + self.h * 0.5)
    }
}

/// Fallback hotspot layout for the no-asset path (synthetic silhouette).
/// Primary L/R click are intentionally absent — Logi doesn't expose them
/// as remappable and we follow the same rule everywhere.
#[must_use]
pub fn default_hotspots(thumbwheel: bool) -> Vec<Hotspot> {
    let mut hotspots = vec![
        Hotspot {
            id: ButtonId::MiddleClick.into(),
            x: 180.,
            y: 110.,
            w: 60.,
            h: 90.,
        },
        Hotspot {
            id: ButtonId::Back.into(),
            x: 0.,
            y: 220.,
            w: 40.,
            h: 60.,
        },
        Hotspot {
            id: ButtonId::Forward.into(),
            x: 0.,
            y: 290.,
            w: 40.,
            h: 60.,
        },
        Hotspot {
            id: ButtonId::DpiToggle.into(),
            x: 175.,
            y: 230.,
            w: 70.,
            h: 40.,
        },
        Hotspot {
            id: ButtonId::GestureButton.into(),
            x: 8.,
            y: 380.,
            w: 44.,
            h: 80.,
        },
    ];
    if thumbwheel {
        hotspots.push(Hotspot {
            id: MouseControlId::ThumbwheelRotation,
            x: 8.,
            y: 140.,
            w: 44.,
            h: 70.,
        });
    }
    hotspots
}

/// Silhouette controls that only `0x1b04` can capture. A [`SpyModel`] has no
/// `0x1b04`, so they come off its silhouette; its extras take their place.
const REPROG_ONLY: [ButtonId; 3] = [
    ButtonId::GestureButton,
    ButtonId::DpiToggle,
    ButtonId::HapticPanel,
];

/// Where an extra button sits on the synthetic silhouette, or `None` for a
/// button the silhouette has no place for.
///
/// Placeholders until the device image places them: the G502 X Plus's G6
/// takes the left-thumb slot, G7/G8 flank the DPI pad, G9 sits behind them
/// and the wheel tilts flank the wheel.
fn silhouette_slot(button: ButtonId) -> Option<(f32, f32, f32, f32)> {
    match button {
        ButtonId::WheelTiltLeft => Some((146., 120., 30., 60.)),
        ButtonId::WheelTiltRight => Some((244., 120., 30., 60.)),
        ButtonId::DpiShift => Some((8., 380., 44., 80.)),
        ButtonId::DpiDown => Some((130., 226., 48., 44.)),
        ButtonId::DpiUp => Some((242., 226., 48., 44.)),
        ButtonId::ProfileCycle => Some((175., 278., 70., 40.)),
        _ => None,
    }
}

/// Swap `hotspots`' `0x1b04`-only controls for `model`'s buttons, placed on
/// the silhouette scaled by `scale`. A button the image already places keeps
/// that place.
pub fn merge_spy_hotspots(hotspots: &mut Vec<Hotspot>, model: &SpyModel, scale: (f32, f32)) {
    hotspots.retain(|hotspot| {
        hotspot
            .id
            .button()
            .is_none_or(|button| !REPROG_ONLY.contains(&button))
    });
    for button in model.buttons() {
        if hotspots.iter().any(|existing| existing.id == button.into()) {
            continue;
        }
        let Some((x, y, w, h)) = silhouette_slot(button) else {
            continue;
        };
        hotspots.push(Hotspot {
            id: button.into(),
            x: x * scale.0,
            y: y * scale.1,
            w: w * scale.0,
            h: h * scale.1,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_thumbwheel_directions_share_one_control() {
        assert_eq!(
            MouseControlId::from_active_button(ButtonId::ThumbwheelScrollUp),
            MouseControlId::ThumbwheelRotation
        );
        assert_eq!(
            MouseControlId::from_active_button(ButtonId::ThumbwheelScrollDown),
            MouseControlId::ThumbwheelRotation
        );
    }

    #[test]
    fn fallback_thumbwheel_is_capability_gated() {
        assert!(
            !default_hotspots(false)
                .iter()
                .any(|hotspot| { hotspot.id == MouseControlId::ThumbwheelRotation })
        );
        assert_eq!(
            default_hotspots(true)
                .iter()
                .filter(|hotspot| hotspot.id == MouseControlId::ThumbwheelRotation)
                .count(),
            1
        );
    }

    #[test]
    fn default_hotspots_expose_the_gesture_button() {
        let hotspots = default_hotspots(false);
        assert!(
            hotspots
                .iter()
                .any(|h| { h.id == MouseControlId::Button(ButtonId::GestureButton) }),
            "the gesture button must be a mappable hotspot in the synthetic model"
        );
    }

    #[test]
    fn default_hotspots_omit_primary_clicks() {
        let hotspots = default_hotspots(false);
        assert!(
            !hotspots.iter().any(|h| {
                matches!(
                    h.id,
                    MouseControlId::Button(ButtonId::LeftClick | ButtonId::RightClick)
                )
            }),
            "primary clicks are not remappable and must stay out of the model"
        );
    }

    fn g502() -> &'static SpyModel {
        SpyModel::for_model_key("14099").expect("the G502 X Plus has a spy row")
    }

    #[test]
    fn a_spy_model_swaps_reprog_only_controls_for_its_extras() {
        let mut hotspots = default_hotspots(false);
        merge_spy_hotspots(&mut hotspots, g502(), (1., 1.));
        for button in g502().buttons() {
            assert_eq!(
                hotspots
                    .iter()
                    .filter(|h| h.id == MouseControlId::Button(button))
                    .count(),
                1,
                "{button:?} needs one hotspot"
            );
        }
        assert!(
            !hotspots
                .iter()
                .any(|h| h.id.button().is_some_and(|b| REPROG_ONLY.contains(&b))),
            "a mouse without 0x1b04 cannot capture the MX-only controls"
        );
        assert!(
            hotspots
                .iter()
                .any(|h| h.id == MouseControlId::Button(ButtonId::Back)),
            "the side buttons stay"
        );
    }

    #[test]
    fn an_extra_the_image_places_keeps_its_place() {
        let placed = Hotspot {
            id: ButtonId::DpiUp.into(),
            x: 1.,
            y: 2.,
            w: 3.,
            h: 4.,
        };
        let mut hotspots = vec![placed];
        merge_spy_hotspots(&mut hotspots, g502(), (2., 2.));
        assert_eq!(
            hotspots
                .iter()
                .filter(|h| h.id == MouseControlId::Button(ButtonId::DpiUp))
                .collect::<Vec<_>>(),
            [&placed]
        );
        let shift = hotspots
            .iter()
            .find(|h| h.id == MouseControlId::Button(ButtonId::DpiShift))
            .expect("G6 placed on the silhouette");
        assert_eq!((shift.x, shift.y, shift.w, shift.h), (16., 760., 88., 160.));
    }
}
