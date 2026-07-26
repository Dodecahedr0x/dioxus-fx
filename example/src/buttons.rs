//! Stories for `dioxus_micro_transitions::buttons`.
//!
//! `AnimatedButton` carries all twelve interactions, so it gets one story per
//! [`ButtonInteraction`] variant. Press a button to swap it to its alternate
//! label and icon.

use crate::stage::{Check, Chevron};
use crate::txt;
use dioxus::prelude::*;
use dioxus_micro_transitions::buttons::*;
use dioxus_showcase::prelude::*;

/// Renders one interaction with the icons and alternate state the gallery uses.
fn demo(interaction: ButtonInteraction, label: String) -> Element {
    rsx! {
        AnimatedButton {
            label: txt(label, "Action"),
            alt_label: "Done",
            interaction,
            alt_color: "#34d399",
            icon: rsx! {
                Chevron {}
            },
            alt_icon: rsx! {
                Check {}
            },
        }
    }
}

/// The leading icon slides out as a trailing icon slides in.
#[story(title = "Buttons/AnimatedButton/SlideArrow", tags = ["buttons"])]
pub fn animated_button_slide_arrow(label: String) -> Element {
    demo(ButtonInteraction::SlideArrow, label)
}

/// The icon swaps upward and two stars pop out around it.
#[story(title = "Buttons/AnimatedButton/Sparkle", tags = ["buttons"])]
pub fn animated_button_sparkle(label: String) -> Element {
    demo(ButtonInteraction::Sparkle, label)
}

/// The icon scales out and the alternate icon scales in.
#[story(title = "Buttons/AnimatedButton/Morph", tags = ["buttons"])]
pub fn animated_button_morph(label: String) -> Element {
    demo(ButtonInteraction::Morph, label)
}

/// Like `Morph`, for swaps that only change colour.
#[story(title = "Buttons/AnimatedButton/ColorMorph", tags = ["buttons"])]
pub fn animated_button_color_morph(label: String) -> Element {
    demo(ButtonInteraction::ColorMorph, label)
}

/// The icon beats once.
#[story(title = "Buttons/AnimatedButton/Pulse", tags = ["buttons"])]
pub fn animated_button_pulse(label: String) -> Element {
    demo(ButtonInteraction::Pulse, label)
}

/// The icon turns a half circle.
#[story(title = "Buttons/AnimatedButton/Rotate", tags = ["buttons"])]
pub fn animated_button_rotate(label: String) -> Element {
    demo(ButtonInteraction::Rotate, label)
}

/// The icon shakes side to side.
#[story(title = "Buttons/AnimatedButton/Shake", tags = ["buttons"])]
pub fn animated_button_shake(label: String) -> Element {
    demo(ButtonInteraction::Shake, label)
}

/// The icon tilts out, the alternate tilts in, and a badge pops on.
#[story(title = "Buttons/AnimatedButton/Ring", tags = ["buttons"])]
pub fn animated_button_ring(label: String) -> Element {
    demo(ButtonInteraction::Ring, label)
}

/// A highlight sweeps across the button, repeating while hovered.
#[story(title = "Buttons/AnimatedButton/Glare", tags = ["buttons"])]
pub fn animated_button_glare(label: String) -> Element {
    demo(ButtonInteraction::Glare, label)
}

/// The label rolls up to reveal a second copy of itself.
#[story(title = "Buttons/AnimatedButton/TextReveal", tags = ["buttons"])]
pub fn animated_button_text_reveal(label: String) -> Element {
    demo(ButtonInteraction::TextReveal, label)
}

/// The button leans toward the pointer.
#[story(title = "Buttons/AnimatedButton/Magnetic", tags = ["buttons"])]
pub fn animated_button_magnetic(label: String) -> Element {
    demo(ButtonInteraction::Magnetic, label)
}

/// A ring expands out of the button's edge and fades.
#[story(title = "Buttons/AnimatedButton/ExpandRing", tags = ["buttons"])]
pub fn animated_button_expand_ring(label: String) -> Element {
    demo(ButtonInteraction::ExpandRing, label)
}

/// A row of links that blurs every sibling of the focused one.
#[story(title = "Buttons/FocusBlurLinks", tags = ["buttons"])]
pub fn focus_blur_links(blur: String) -> Element {
    rsx! {
        FocusBlurLinks {
            blur: txt(blur, "2px"),
            items: vec![
                ("@X".into(), "#".into()),
                ("@Threads".into(), "#".into()),
                ("@GitHub".into(), "#".into()),
            ],
        }
    }
}
