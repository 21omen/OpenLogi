//! Geometry helpers for the centre mouse model.
//!
//! These functions keep Logitech asset coordinate translation and fallback
//! label layout separate from the GPUI element tree in `view`.

use openlogi_assets::{Assignment, ImageEntry, Origin, Point};
use openlogi_core::binding::ButtonId;

use super::hotspots::{Hotspot, MOUSE_MODEL_SIZE, MouseControlId};
use super::leader_lines::{Label, Side};
use crate::services::assets::{ResolvedAsset, SideRender};

/// Approx pixel width of each hotspot hit-target. Logitech only gives us a
/// marker point per button, not a rectangle, so we size by hand.
const ASSET_HOTSPOT: f32 = 56.;

/// How far, as a fraction of the PNG's height, the metadata `origin.height`
/// may drift from the real PNG before [`asset_hotspots_for_png`] treats the
/// whole `origin` block as a stale placeholder and stops cropping by it.
const ORIGIN_HEIGHT_TOLERANCE: f32 = 0.05;

/// Height of a side-label card. The layout needs it to group related cards
/// without allowing them to overlap at the minimum model height.
pub(super) const LABEL_H: f32 = 56.;

/// Empty space between the grouped Back and Forward cards when the viewport
/// has enough room to pull them closer than the regular even spacing.
const NAVIGATION_GROUP_GAP: f32 = 16.;

/// Whether label cards occupy one or both sides of the device render.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelDistribution {
    LeftOnly,
    BothSides,
}

/// Scale the device image to *fit inside* a `max_w` × `target_h` box while
/// preserving the **actual PNG's** aspect ratio. A tall device (a mouse) is
/// bound by the height; a wide one (a keyboard) is bound by the width — which
/// is what stops a wide keyboard render from overflowing the panel (#272).
///
/// The metadata's `origin` reports the silhouette bbox inside the PNG, which
/// is typically narrower than the full image (Logi pads transparent strips on
/// both sides); sizing by origin causes `ObjectFit::Contain` to letterbox
/// vertically and pulls every hotspot off the rendered button.
#[expect(
    clippy::cast_precision_loss,
    reason = "device images are < 4096 px on either axis — well within f32 mantissa"
)]
pub fn asset_dimensions_for_png(asset: &ResolvedAsset, target_h: f32, max_w: f32) -> (f32, f32) {
    if asset.png_height == 0 {
        return MOUSE_MODEL_SIZE;
    }
    let aspect = (asset.png_width as f32) / (asset.png_height as f32);
    let w = target_h * aspect;
    if w > max_w {
        (max_w, max_w / aspect)
    } else {
        (w, target_h)
    }
}

/// Gap between a depot's side view and its main render in the model strip.
const SIDE_VIEW_GAP: f32 = 24.;

/// One render's place in the model strip, in model-local pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StripImage {
    pub x: f32,
    pub w: f32,
}

/// How the mouse model lays out an asset's renders: its main render and,
/// when the depot marks buttons on one, its side view on the left.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModelStrip {
    pub side: Option<StripImage>,
    pub main: StripImage,
    pub w: f32,
    pub h: f32,
}

/// The main render's button markers: Options+ marks them on a dedicated
/// `device_buttons_image` entry, a G-series depot (G HUB metadata) on the
/// `device_image` entry itself.
fn main_markers(asset: &ResolvedAsset) -> Option<&ImageEntry> {
    let entry = |key: &str| {
        asset.metadata.images.iter().find(|image| {
            image.key == key
                && image
                    .assignments
                    .iter()
                    .any(|a| map_assignment(a).is_some())
        })
    };
    entry("device_buttons_image").or_else(|| entry("device_image"))
}

/// The side view's button markers, when the depot ships the view and marks
/// buttons on it.
fn side_markers(asset: &ResolvedAsset) -> Option<(&ImageEntry, &SideRender)> {
    let side = asset.side.as_ref()?;
    let entry = asset
        .metadata
        .images
        .iter()
        .find(|image| image.key == "device_side")
        .filter(|image| {
            image
                .assignments
                .iter()
                .any(|a| map_assignment(a).is_some())
        })?;
    Some((entry, side))
}

/// Whether the asset exposes any remappable button markers. Mice do (so the
/// model reserves a side gutter for their leader-line labels); keyboards and
/// other label-less devices don't, so the model can hand them the full width.
pub fn asset_has_button_labels(asset: &ResolvedAsset) -> bool {
    main_markers(asset).is_some() || side_markers(asset).is_some()
}

/// Fit the asset's renders inside a `max_w` × `target_h` box: the main render
/// alone as [`asset_dimensions_for_png`] fits it, or, when the depot marks
/// buttons on its side view, both side by side at one height.
#[expect(
    clippy::cast_precision_loss,
    reason = "device images are < 4096 px on either axis — well within f32 mantissa"
)]
pub fn asset_strip(asset: &ResolvedAsset, target_h: f32, max_w: f32) -> ModelStrip {
    let side = side_markers(asset)
        .map(|(_, side)| side)
        .filter(|side| side.png_height > 0 && asset.png_height > 0);
    let Some(side) = side else {
        let (w, h) = asset_dimensions_for_png(asset, target_h, max_w);
        return ModelStrip {
            side: None,
            main: StripImage { x: 0., w },
            w,
            h,
        };
    };
    let side_aspect = side.png_width as f32 / side.png_height as f32;
    let main_aspect = asset.png_width as f32 / asset.png_height as f32;
    let h = target_h.min((max_w - SIDE_VIEW_GAP).max(0.) / (side_aspect + main_aspect));
    let side_w = h * side_aspect;
    let main_w = h * main_aspect;
    ModelStrip {
        side: Some(StripImage { x: 0., w: side_w }),
        main: StripImage {
            x: side_w + SIDE_VIEW_GAP,
            w: main_w,
        },
        w: side_w + SIDE_VIEW_GAP + main_w,
        h,
    }
}

/// Every button marker of the asset as a hotspot in `strip`'s model-local
/// pixels: the main render's, and the side view's when `strip` shows it.
///
/// Primary left/right clicks deliberately have no entry — Logi never
/// exposes them as remappable (and Options+ doesn't either), so we don't
/// invent markers for them.
pub fn asset_hotspots(asset: &ResolvedAsset, strip: &ModelStrip) -> Vec<Hotspot> {
    let mut hotspots = Vec::new();
    if let Some(entry) = main_markers(asset) {
        // Options+ depots share one origin across their entries; the first
        // one is the silhouette's.
        let origin = asset.metadata.origin().unwrap_or(entry.origin);
        hotspots.extend(place_markers(
            entry,
            origin,
            (asset.png_width, asset.png_height),
            strip.main,
            strip.h,
        ));
    }
    if let (Some((entry, render)), Some(place)) = (side_markers(asset), strip.side) {
        hotspots.extend(place_markers(
            entry,
            entry.origin,
            (render.png_width, render.png_height),
            place,
            strip.h,
        ));
    }
    hotspots
}

/// Convert one image entry's markers into hotspots on its render, drawn at
/// `place` and `render_h`, translating from the metadata's "origin" coord
/// system (the silhouette bbox) into the actual rendered PNG coord system.
///
/// Options+ markers are percentages of `origin`; G HUB markers are pixels of
/// it, told apart by any coordinate beyond 100 (as on the Keys tab). Within
/// the actual PNG, that bbox is centred with equal padding on the left and
/// right. We render at the *PNG's* full aspect (no letterboxing) so the
/// marker translation is:
///
/// ```text
/// bbox_w_rendered = render_w * origin.width  / png.width
/// bbox_x_offset   = (render_w - bbox_w_rendered) / 2
/// hotspot.x       = bbox_x_offset + marker.x / 100 * bbox_w_rendered
/// hotspot.y       = marker.y / 100 * render_h     // height ratio is 1:1
/// ```
///
/// The `hotspot.y` line only holds if `origin.height` actually equals the
/// PNG's height — some depots (seen on the M720 Triathlon: `origin` is a
/// stale `396×396` placeholder reused across every image entry, against a
/// real `1228×1920` PNG) violate that, and trusting `origin.width` there
/// crushes every hotspot into a narrow sliver instead of spreading them
/// across the render. When `origin.height` is not within
/// [`ORIGIN_HEIGHT_TOLERANCE`] of the PNG's height, the whole `origin` block
/// is untrustworthy, so we fall back to treating it as the full PNG (no
/// crop) rather than compressing hotspots with a bogus width.
#[expect(
    clippy::cast_precision_loss,
    reason = "device images are < 4096 px on either axis — well within f32 mantissa"
)]
fn place_markers(
    entry: &ImageEntry,
    origin: Origin,
    (png_w, png_h): (u32, u32),
    place: StripImage,
    render_h: f32,
) -> Vec<Hotspot> {
    let png_w = png_w as f32;
    let png_h = png_h as f32;
    let trusted = (png_h > 0.
        && (origin.height as f32 - png_h).abs() <= png_h * ORIGIN_HEIGHT_TOLERANCE)
        .then_some(origin);
    let origin_w = trusted.map_or(png_w, |o| o.width as f32).min(png_w);
    let bbox_w_rendered = if png_w > 0. {
        place.w * origin_w / png_w
    } else {
        place.w
    };
    let bbox_x_offset = (place.w - bbox_w_rendered) / 2.;
    let pixels = entry
        .assignments
        .iter()
        .any(|a| a.marker.x > 100. || a.marker.y > 100.);
    // A pixel marker is a fraction of the authored canvas; a stale origin
    // leaves the PNG as that canvas.
    let (canvas_w, canvas_h) =
        trusted.map_or((png_w, png_h), |o| (o.width as f32, o.height as f32));
    let percent = |marker: Point| {
        if pixels {
            (
                marker.x / canvas_w.max(1.) * 100.,
                marker.y / canvas_h.max(1.) * 100.,
            )
        } else {
            (marker.x, marker.y)
        }
    };
    entry
        .assignments
        .iter()
        .filter_map(|a| {
            let id = map_assignment(a)?;
            let (mx, my) = percent(a.marker);
            let cx = place.x + bbox_x_offset + mx / 100. * bbox_w_rendered;
            let cy = my / 100. * render_h;
            Some(Hotspot {
                id,
                x: cx - ASSET_HOTSPOT / 2.,
                y: cy - ASSET_HOTSPOT / 2.,
                w: ASSET_HOTSPOT,
                h: ASSET_HOTSPOT,
            })
        })
        .collect()
}

/// Lay labels out evenly down one or both sides of the mouse. A two-sided
/// layout sends the leftmost half of the hotspots left and the rightmost half
/// right, then orders each side by hotspot height. Back and Forward stay
/// adjacent when both are on the same side because they form one navigation
/// pair, even when another marker sits between them.
#[expect(
    clippy::cast_precision_loss,
    reason = "hotspot count is bounded by ButtonId variants — well under f32 mantissa"
)]
pub fn labels_from_hotspots(
    hotspots: &[Hotspot],
    mouse_h: f32,
    distribution: LabelDistribution,
) -> Vec<Label> {
    if hotspots.is_empty() {
        return Vec::new();
    }

    let mut labels: Vec<Label> = hotspots
        .iter()
        .map(|hotspot| Label {
            id: hotspot.id,
            side: Side::Left,
            y: 0.,
        })
        .collect();
    if distribution == LabelDistribution::BothSides {
        let mut horizontal_order: Vec<usize> = (0..hotspots.len()).collect();
        horizontal_order
            .sort_by(|&a, &b| hotspots[a].center().0.total_cmp(&hotspots[b].center().0));
        for index in horizontal_order
            .into_iter()
            .skip(hotspots.len().div_ceil(2))
        {
            labels[index].side = Side::Right;
        }
    }

    for side in [Side::Left, Side::Right] {
        let mut vertical_order: Vec<usize> = labels
            .iter()
            .enumerate()
            .filter_map(|(index, label)| (label.side == side).then_some(index))
            .collect();
        vertical_order.sort_by(|&a, &b| hotspots[a].center().1.total_cmp(&hotspots[b].center().1));
        let back = vertical_order
            .iter()
            .position(|&index| labels[index].id == ButtonId::Back.into());
        let forward = vertical_order
            .iter()
            .position(|&index| labels[index].id == ButtonId::Forward.into());
        let navigation_pair = if let (Some(back), Some(forward)) = (back, forward) {
            let first = back.min(forward);
            let second = back.max(forward);
            if second > first + 1 {
                let navigation_button = vertical_order.remove(second);
                vertical_order.insert(first + 1, navigation_button);
            }
            Some((vertical_order[first], vertical_order[first + 1]))
        } else {
            None
        };
        let step = mouse_h / (vertical_order.len() as f32 + 1.);
        for (slot, index) in vertical_order.into_iter().enumerate() {
            labels[index].y = step * (slot as f32 + 1.);
        }
        if let Some((first, second)) = navigation_pair {
            let grouped_step = step.min(LABEL_H + NAVIGATION_GROUP_GAP);
            let adjustment = (step - grouped_step) / 2.;
            labels[first].y += adjustment;
            labels[second].y -= adjustment;
        }
    }

    labels
}

/// Label positions for the synthetic fallback silhouette.
pub fn default_labels(thumbwheel: bool, distribution: LabelDistribution) -> Vec<Label> {
    labels_from_hotspots(
        &super::hotspots::default_hotspots(thumbwheel),
        MOUSE_MODEL_SIZE.1,
        distribution,
    )
}

/// The control a marker stands for: by Logitech's slot name, or, on a
/// G-series depot whose metadata names no slots, by its slot id.
fn map_assignment(assignment: &Assignment) -> Option<MouseControlId> {
    map_slot_name(&assignment.slot_name).or_else(|| map_slot_id(&assignment.slot_id))
}

/// G-series slot ids → OpenLogi's visual control IDs.
///
/// These depots number their slots (`<depot>_g<n>_m1`) by neither the
/// button's function nor the number printed on it, so each depot is mapped
/// by where its markers sit on its renders. G502 X Plus (`g502x-plus`): on
/// the top view `g3` is the wheel click, `g7`/`g8` flank the wheel (its
/// tilt), `g9` sits behind it, and `g10`/`g11` sit on the buttons printed
/// "G8" and "G7" left of the left click; on the side view `g4` is the rear
/// thumb button printed "G4", `g6` the middle one printed "G5" and `g5` the
/// front blade, the sniper button.
fn map_slot_id(id: &str) -> Option<MouseControlId> {
    let button = match id {
        "g502x-plus_g3_m1" => ButtonId::MiddleClick,
        "g502x-plus_g4_m1" => ButtonId::Back,
        "g502x-plus_g5_m1" => ButtonId::DpiShift,
        "g502x-plus_g6_m1" => ButtonId::Forward,
        "g502x-plus_g7_m1" => ButtonId::WheelTiltLeft,
        "g502x-plus_g8_m1" => ButtonId::WheelTiltRight,
        "g502x-plus_g9_m1" => ButtonId::ProfileCycle,
        "g502x-plus_g10_m1" => ButtonId::DpiUp,
        "g502x-plus_g11_m1" => ButtonId::DpiDown,
        _ => return None,
    };
    Some(MouseControlId::Button(button))
}

/// Logitech's stable slot vocabulary → OpenLogi's visual control IDs. Intentionally
/// conservative; unknown names fall through so widening `MouseControlId` later
/// doesn't break old depots.
fn map_slot_name(name: &str) -> Option<MouseControlId> {
    match name {
        "SLOT_NAME_LEFT_BUTTON" => Some(MouseControlId::Button(ButtonId::LeftClick)),
        "SLOT_NAME_RIGHT_BUTTON" => Some(MouseControlId::Button(ButtonId::RightClick)),
        "SLOT_NAME_MIDDLE_BUTTON" => Some(MouseControlId::Button(ButtonId::MiddleClick)),
        // The main wheel's tilt. Logi names the two slots after the scroll they
        // produce in firmware; each is its own reprogrammable control
        // (`0x1b04` CIDs `0x005b` / `0x005d`), not part of the middle click.
        "SLOT_NAME_LEFT_SCROLL_BUTTON" | "SLOT_NAME_SCROLL_LEFT" => {
            Some(MouseControlId::Button(ButtonId::WheelTiltLeft))
        }
        "SLOT_NAME_RIGHT_SCROLL_BUTTON" | "SLOT_NAME_SCROLL_RIGHT" => {
            Some(MouseControlId::Button(ButtonId::WheelTiltRight))
        }
        "SLOT_NAME_BACK_BUTTON" => Some(MouseControlId::Button(ButtonId::Back)),
        "SLOT_NAME_FORWARD_BUTTON" => Some(MouseControlId::Button(ButtonId::Forward)),
        // MX Ergo (CID 0x00ED) and MX Ergo S (CID 0x00FD) name their
        // precision-mode/DPI-shift button after its Options+ function.
        "SLOT_NAME_MODESHIFT_BUTTON"
        | "SLOT_NAME_DPI_BUTTON"
        | "SLOT_NAME_PRECISION_MODE_BUTTON"
        | "SLOT_NAME_CHANGE_POINTER_SPEED" => Some(MouseControlId::Button(ButtonId::DpiToggle)),
        "SLOT_NAME_THUMBWHEEL" => Some(MouseControlId::ThumbwheelRotation),
        "SLOT_NAME_GESTURE_BUTTON" => Some(MouseControlId::Button(ButtonId::GestureButton)),
        // The MX Master 4 Haptic Sense Panel. Logi names the slot after its
        // Options+ default assignment (the radial Actions Ring menu), but the
        // marker is the panel itself.
        "ASSIGNMENT_NAME_SHOW_RADIAL_MENU" => Some(MouseControlId::Button(ButtonId::HapticPanel)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use openlogi_assets::{Assignment, Direction, ImageEntry, Metadata, Origin, Point};
    use openlogi_core::device::DeviceKind;

    use super::*;
    use crate::features::mouse::hotspots::default_hotspots;

    #[test]
    fn default_labels_include_capability_gated_thumbwheel() {
        assert!(
            !default_labels(false, LabelDistribution::LeftOnly)
                .iter()
                .any(|label| label.id == MouseControlId::ThumbwheelRotation)
        );
        assert_eq!(
            default_labels(true, LabelDistribution::LeftOnly)
                .iter()
                .filter(|label| label.id == MouseControlId::ThumbwheelRotation)
                .count(),
            1
        );
    }

    #[test]
    fn thumbwheel_metadata_maps_to_one_rotation_control() {
        assert_eq!(
            map_slot_name("SLOT_NAME_THUMBWHEEL"),
            Some(MouseControlId::ThumbwheelRotation)
        );
    }

    #[test]
    fn dpi_slot_names_map_to_dpi_toggle_button() {
        assert_eq!(
            map_slot_name("SLOT_NAME_MODESHIFT_BUTTON"),
            Some(MouseControlId::Button(ButtonId::DpiToggle))
        );
        assert_eq!(
            map_slot_name("SLOT_NAME_DPI_BUTTON"),
            Some(MouseControlId::Button(ButtonId::DpiToggle))
        );
        // MX Ergo S and MX Ergo name their precision-mode button differently.
        assert_eq!(
            map_slot_name("SLOT_NAME_CHANGE_POINTER_SPEED"),
            Some(MouseControlId::Button(ButtonId::DpiToggle))
        );
        assert_eq!(
            map_slot_name("SLOT_NAME_PRECISION_MODE_BUTTON"),
            Some(MouseControlId::Button(ButtonId::DpiToggle))
        );
    }

    #[test]
    fn wheel_tilt_slot_names_map_to_their_own_controls() {
        // MX Anywhere uses the longer names; MX Ergo uses the shorter aliases.
        for name in ["SLOT_NAME_LEFT_SCROLL_BUTTON", "SLOT_NAME_SCROLL_LEFT"] {
            assert_eq!(
                map_slot_name(name),
                Some(MouseControlId::Button(ButtonId::WheelTiltLeft))
            );
        }
        for name in ["SLOT_NAME_RIGHT_SCROLL_BUTTON", "SLOT_NAME_SCROLL_RIGHT"] {
            assert_eq!(
                map_slot_name(name),
                Some(MouseControlId::Button(ButtonId::WheelTiltRight))
            );
        }
    }

    #[test]
    fn labels_track_hotspots_and_avoid_crossing() {
        let hotspots = default_hotspots(true);
        let labels =
            labels_from_hotspots(&hotspots, MOUSE_MODEL_SIZE.1, LabelDistribution::LeftOnly);
        assert_eq!(labels.len(), hotspots.len());

        let mut ys: Vec<f32> = labels.iter().map(|l| l.y).collect();
        ys.sort_by(f32::total_cmp);
        ys.dedup();
        assert_eq!(ys.len(), labels.len(), "each label gets a distinct slot");
    }

    #[test]
    fn navigation_labels_stay_together_when_haptic_marker_sits_between() {
        let hotspots = [
            Hotspot {
                id: ButtonId::Forward.into(),
                x: 0.,
                y: 100.,
                w: 10.,
                h: 10.,
            },
            Hotspot {
                id: ButtonId::HapticPanel.into(),
                x: 0.,
                y: 200.,
                w: 10.,
                h: 10.,
            },
            Hotspot {
                id: ButtonId::Back.into(),
                x: 0.,
                y: 300.,
                w: 10.,
                h: 10.,
            },
        ];

        let mut labels =
            labels_from_hotspots(&hotspots, MOUSE_MODEL_SIZE.1, LabelDistribution::LeftOnly);
        labels.sort_by(|a, b| a.y.total_cmp(&b.y));

        assert_eq!(
            labels.iter().map(|label| label.id).collect::<Vec<_>>(),
            [
                MouseControlId::Button(ButtonId::Forward),
                MouseControlId::Button(ButtonId::Back),
                MouseControlId::Button(ButtonId::HapticPanel),
            ]
        );
        let navigation_gap = labels[1].y - labels[0].y;
        let haptic_gap = labels[2].y - labels[1].y;
        assert!(navigation_gap < haptic_gap);
        assert!(navigation_gap >= LABEL_H);
    }

    #[test]
    fn a_two_sided_layout_uses_both_sides() {
        let hotspots = default_hotspots(true);
        let labels =
            labels_from_hotspots(&hotspots, MOUSE_MODEL_SIZE.1, LabelDistribution::BothSides);

        assert!(labels.iter().any(|label| label.side == Side::Left));
        assert!(labels.iter().any(|label| label.side == Side::Right));
    }

    fn mouse_asset(
        origin: (u32, u32),
        png: (u32, u32),
        markers: &[(&str, f32, f32)],
    ) -> ResolvedAsset {
        let assignments = markers
            .iter()
            .map(|(slot_name, x, y)| Assignment {
                slot_id: String::new(),
                slot_name: (*slot_name).to_string(),
                marker: Point { x: *x, y: *y },
                label: Direction { x: 0, y: 0 },
            })
            .collect();
        ResolvedAsset {
            depot: "m720_triathlon".to_string(),
            display_name: "M720 Triathlon".to_string(),
            kind: Some(DeviceKind::Mouse),
            image_path: PathBuf::from("/tmp/side.png"),
            hero_image_path: None,
            glow: None,
            metadata: Metadata {
                images: vec![
                    ImageEntry {
                        key: "device_image".to_string(),
                        origin: Origin {
                            width: origin.0,
                            height: origin.1,
                        },
                        assignments: Vec::new(),
                    },
                    ImageEntry {
                        key: "device_buttons_image".to_string(),
                        origin: Origin {
                            width: origin.0,
                            height: origin.1,
                        },
                        assignments,
                    },
                ],
            },
            png_width: png.0,
            png_height: png.1,
            side: None,
        }
    }

    #[test]
    fn stale_placeholder_origin_falls_back_to_the_full_png_width() {
        // Reproduces the M720 Triathlon depot: every image entry carries a
        // stale 396x396 placeholder `origin` against a real 1228x1920 PNG.
        // Trusting `origin.width` there crushes every hotspot into a
        // ~32%-wide sliver centred on the render instead of spreading them
        // across it.
        let asset = mouse_asset(
            (396, 396),
            (1228, 1920),
            &[("SLOT_NAME_MIDDLE_BUTTON", 66., 14.)],
        );

        let hotspots = asset_hotspots(&asset, &plain_strip(300., 470.));

        assert_eq!(hotspots.len(), 1);
        let (cx, _) = hotspots[0].center();
        assert!((cx - 0.66 * 300.).abs() < 1., "cx={cx}");
    }

    #[test]
    fn a_correct_origin_still_applies_the_horizontal_crop() {
        // A depot whose `origin.height` genuinely matches the PNG (Logi pads
        // transparent strips on the sides only) must still get the crop
        // correction — the fallback must not fire on valid data. The marker
        // sits off-centre: a centred one lands on the same pixel with or
        // without the crop, so it could not catch a broken correction.
        let asset = mouse_asset(
            (900, 1920),
            (1228, 1920),
            &[("SLOT_NAME_MIDDLE_BUTTON", 25., 14.)],
        );

        let hotspots = asset_hotspots(&asset, &plain_strip(300., 470.));

        assert_eq!(hotspots.len(), 1);
        let (cx, _) = hotspots[0].center();
        let bbox_w = 300. * 900. / 1228.;
        let expected = (300. - bbox_w) / 2. + 0.25 * bbox_w;
        assert!((cx - expected).abs() < 1., "cx={cx}");
    }

    /// A strip of the main render alone, `w` × `h`.
    fn plain_strip(w: f32, h: f32) -> ModelStrip {
        ModelStrip {
            side: None,
            main: StripImage { x: 0., w },
            w,
            h,
        }
    }

    fn marker(slot_id: &str, x: f32, y: f32) -> Assignment {
        Assignment {
            slot_id: slot_id.to_string(),
            slot_name: String::new(),
            marker: Point { x, y },
            label: Direction { x: 0, y: 0 },
        }
    }

    /// The G502 X Plus depot as it ships: G HUB pixel markers against a
    /// 1391×2700 canvas on the top view (`device_image`, a 1661×2800 render)
    /// and a 936×2700 canvas on the side view (a 1153×2800 render).
    fn g502_asset(side_cached: bool) -> ResolvedAsset {
        ResolvedAsset {
            depot: "g502x_plus".to_string(),
            display_name: "G502 X PLUS".to_string(),
            kind: Some(DeviceKind::Mouse),
            image_path: PathBuf::from("/tmp/top.png"),
            hero_image_path: None,
            glow: None,
            metadata: Metadata {
                images: vec![
                    ImageEntry {
                        key: "device_image".to_string(),
                        origin: Origin {
                            width: 1391,
                            height: 2700,
                        },
                        assignments: vec![
                            marker("g502x-plus_g1_m1", 480., 450.),
                            marker("g502x-plus_g3_m1", 800., 680.),
                            marker("g502x-plus_g7_m1", 700., 680.),
                            marker("g502x-plus_g8_m1", 900., 680.),
                            marker("g502x-plus_g9_m1", 800., 1250.),
                            marker("g502x-plus_g10_m1", 300., 500.),
                            marker("g502x-plus_g11_m1", 250., 950.),
                            marker("g502x-plus_scroll1_m1", 800., 500.),
                        ],
                    },
                    ImageEntry {
                        key: "device_side".to_string(),
                        origin: Origin {
                            width: 936,
                            height: 2700,
                        },
                        assignments: vec![
                            marker("g502x-plus_g4_m1", 580., 1800.),
                            marker("g502x-plus_g5_m1", 260., 1000.),
                            marker("g502x-plus_g6_m1", 500., 1400.),
                        ],
                    },
                ],
            },
            png_width: 1661,
            png_height: 2800,
            side: side_cached.then(|| SideRender {
                path: PathBuf::from("/tmp/side.png"),
                png_width: 1153,
                png_height: 2800,
            }),
        }
    }

    fn center_of(hotspots: &[Hotspot], button: ButtonId) -> (f32, f32) {
        hotspots
            .iter()
            .find(|hotspot| hotspot.id == MouseControlId::Button(button))
            .unwrap_or_else(|| panic!("{button:?} has a hotspot"))
            .center()
    }

    /// Pixel markers are fractions of the authored canvas, centred in the
    /// render like percent markers: the DPI-up button's marker (300, 500)
    /// lands where the depot's own render puts it.
    #[test]
    fn g_series_pixel_markers_land_on_their_buttons() {
        let asset = g502_asset(false);
        let strip = asset_strip(&asset, 560., 1000.);
        assert_eq!(strip.side, None, "no side view without its render");
        let hotspots = asset_hotspots(&asset, &strip);
        let (cx, cy) = center_of(&hotspots, ButtonId::DpiUp);
        let bbox_w = strip.main.w * 1391. / 1661.;
        assert!((cx - ((strip.main.w - bbox_w) / 2. + 300. / 1391. * bbox_w)).abs() < 0.5);
        assert!((cy - 500. / 2700. * strip.h).abs() < 0.5);
        let tilt_left = center_of(&hotspots, ButtonId::WheelTiltLeft);
        let middle = center_of(&hotspots, ButtonId::MiddleClick);
        let tilt_right = center_of(&hotspots, ButtonId::WheelTiltRight);
        assert!(tilt_left.0 < middle.0 && middle.0 < tilt_right.0);
        center_of(&hotspots, ButtonId::ProfileCycle);
        center_of(&hotspots, ButtonId::DpiDown);
        assert!(
            !hotspots
                .iter()
                .any(|hotspot| matches!(hotspot.id, MouseControlId::Button(ButtonId::LeftClick))),
            "unmapped slots — the primary click, the wheel rotation — get no hotspot"
        );
        assert_eq!(hotspots.len(), 6);
    }

    /// The thumb buttons are marked on the side view only: it sits left of
    /// the top view at the same height, and its markers land on it.
    #[test]
    fn the_side_view_carries_the_thumb_buttons() {
        let asset = g502_asset(true);
        assert!(asset_has_button_labels(&asset));
        let strip = asset_strip(&asset, 560., 1000.);
        let side = strip.side.expect("the side view is shown");
        assert_eq!(side.x, 0.);
        assert!((strip.main.x - (side.w + SIDE_VIEW_GAP)).abs() < 0.01);
        assert!((side.w - strip.h * 1153. / 2800.).abs() < 0.01);
        assert!((strip.main.w - strip.h * 1661. / 2800.).abs() < 0.01);
        let hotspots = asset_hotspots(&asset, &strip);
        assert_eq!(hotspots.len(), 9);
        for button in [ButtonId::Back, ButtonId::Forward, ButtonId::DpiShift] {
            let (cx, _) = center_of(&hotspots, button);
            assert!(cx < side.w, "{button:?} sits on the side view");
        }
        // Front blade above the middle button above the rear one.
        let shift = center_of(&hotspots, ButtonId::DpiShift).1;
        let forward = center_of(&hotspots, ButtonId::Forward).1;
        let back = center_of(&hotspots, ButtonId::Back).1;
        assert!(shift < forward && forward < back);
        let (dpi_x, _) = center_of(&hotspots, ButtonId::DpiUp);
        assert!(dpi_x > strip.main.x, "the top view's markers move with it");
    }

    #[test]
    fn a_narrow_box_shrinks_both_views_to_fit() {
        let asset = g502_asset(true);
        let strip = asset_strip(&asset, 560., 300.);
        assert!((strip.w - 300.).abs() < 0.01);
        assert!(strip.h < 560.);
    }

    #[test]
    fn unknown_slot_ids_have_no_control() {
        assert_eq!(map_slot_id("g502x-plus_g1_m1"), None);
        assert_eq!(map_slot_id("g502x-plus_scroll1_m1"), None);
        assert_eq!(map_slot_id("g513_g1_m1"), None);
    }
}
