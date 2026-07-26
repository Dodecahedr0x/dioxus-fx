//! Stories for `dioxus_fx::hover`.
//!
//! These react to the pointer over their own element, so move the cursor
//! across the preview.

use dioxus::prelude::*;
use dioxus_fx::hover::*;
use dioxus_showcase::prelude::*;

/// A button with a glow that tracks the pointer across its face.
#[story(title = "Hover/GlowButton", tags = ["hover"])]
pub fn glow_button(
    #[default = "rgba(59,130,246,.15)"] glow_color: String,
    #[default = 120.0] glow_size: f64,
) -> Element {
    rsx! {
        GlowButton { glow_color: glow_color, glow_size: glow_size,
            "Hover me"
        }
    }
}

/// A button that leans toward the pointer while it is nearby.
#[story(title = "Hover/MagneticButton", tags = ["hover"])]
pub fn magnetic_button(#[default = 45.0] range: f64, #[default = 0.35] strength: f64) -> Element {
    rsx! {
        MagneticButton { range: range, strength: strength,
            "Come closer"
        }
    }
}

/// A card that tilts in 3D toward the pointer.
#[story(title = "Hover/TiltCard", tags = ["hover"])]
pub fn tilt_card(#[default = 15.0] max_tilt: f64) -> Element {
    rsx! {
        TiltCard { max_tilt: max_tilt,
            div { style: "padding:24px 32px;font:600 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
                "Tilt me"
            }
        }
    }
}

/// A grid of cards with a highlight that slides in behind the hovered one.
#[story(title = "Hover/CardHover", tags = ["hover"])]
pub fn card_hover(
    #[default = 3] columns: usize,
    #[default = "rgba(128,128,128,.18)"] highlight_color: String,
) -> Element {
    rsx! {
        CardHover {
            columns,
            highlight_color,
            items: vec![
                CardHoverItem::new("Compositor-driven", "Every animation is CSS keyframes."),
                CardHoverItem::new("One dependency", "Just dioxus. No motion library."),
                CardHoverItem::new("Nothing to set up", "Components inject their own CSS."),
            ],
        }
    }
}
