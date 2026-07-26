//! Stories for `dioxus_micro_transitions::hover`.
//!
//! These react to the pointer over their own element, so move the cursor
//! across the preview.

use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_micro_transitions::hover::*;
use dioxus_showcase::prelude::*;

/// A button with a glow that tracks the pointer across its face.
#[story(title = "Hover/GlowButton", tags = ["hover"])]
pub fn glow_button(glow_color: String, glow_size: f64) -> Element {
    rsx! {
        GlowButton { glow_color: txt(glow_color, "rgba(59,130,246,.15)"), glow_size: num(glow_size, 120.0),
            "Hover me"
        }
    }
}

/// A button that leans toward the pointer while it is nearby.
#[story(title = "Hover/MagneticButton", tags = ["hover"])]
pub fn magnetic_button(range: f64, strength: f64) -> Element {
    rsx! {
        MagneticButton { range: num(range, 45.0), strength: num(strength, 0.35),
            "Come closer"
        }
    }
}

/// A card that tilts in 3D toward the pointer.
#[story(title = "Hover/TiltCard", tags = ["hover"])]
pub fn tilt_card(max_tilt: f64) -> Element {
    rsx! {
        TiltCard { max_tilt: num(max_tilt, 15.0),
            div { style: "padding:24px 32px;font:600 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
                "Tilt me"
            }
        }
    }
}

/// A grid of cards with a highlight that slides in behind the hovered one.
#[story(title = "Hover/CardHover", tags = ["hover"])]
pub fn card_hover(columns: usize, highlight_color: String) -> Element {
    rsx! {
        CardHover {
            columns: num(columns, 3),
            highlight_color: txt(highlight_color, "rgba(128,128,128,.18)"),
            items: vec![
                CardHoverItem::new("Compositor-driven", "Every animation is CSS keyframes."),
                CardHoverItem::new("One dependency", "Just dioxus. No motion library."),
                CardHoverItem::new("Nothing to set up", "Components inject their own CSS."),
            ],
        }
    }
}
