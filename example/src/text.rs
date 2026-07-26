//! Stories for `dioxus_micro_transitions::text`.
//!
//! Staggered reveals, split by character, word or line. Type into the `text`
//! control to replay one with your own copy.

use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_micro_transitions::text::*;
use dioxus_showcase::prelude::*;

/// Reveals text character by character out of a blur.
#[story(title = "Text/BlurText", tags = ["text"])]
pub fn blur_text(text: String, duration: f64, stagger_delay: f64, initial_blur: String) -> Element {
    rsx! {
        BlurText {
            text: txt(text, "Blur reveal"),
            duration: num(duration, 0.5),
            stagger_delay: num(stagger_delay, 0.02),
            initial_blur: txt(initial_blur, "8px"),
        }
    }
}

/// Lifts text into place one character at a time.
#[story(title = "Text/CharacterStagger", tags = ["text"])]
pub fn character_stagger(
    text: String,
    duration: f64,
    stagger_delay: f64,
    y_offset: f64,
) -> Element {
    rsx! {
        CharacterStagger {
            text: txt(text, "Character stagger"),
            duration: num(duration, 0.4),
            stagger_delay: num(stagger_delay, 0.015),
            y_offset: num(y_offset, 15.0),
        }
    }
}

/// Reveals text one word at a time.
#[story(title = "Text/WordReveal", tags = ["text"])]
pub fn word_reveal(text: String, duration: f64, stagger_delay: f64) -> Element {
    rsx! {
        WordReveal {
            text: txt(text, "One word at a time"),
            duration: num(duration, 0.5),
            stagger_delay: num(stagger_delay, 0.04),
        }
    }
}

/// Slides each line up from behind a mask.
#[story(title = "Text/TextReveal", tags = ["text"])]
pub fn text_reveal(text: String, duration: f64, stagger_delay: f64) -> Element {
    rsx! {
        TextReveal {
            text: txt(text, "Line by line\nfrom behind a mask"),
            duration: num(duration, 0.8),
            stagger_delay: num(stagger_delay, 0.15),
        }
    }
}
