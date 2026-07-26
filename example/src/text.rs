//! Stories for `dioxus_fx::text`.
//!
//! Staggered reveals, split by character, word or line. Type into the `text`
//! control to replay one with your own copy.

use dioxus::prelude::*;
use dioxus_fx::text::*;
use dioxus_showcase::prelude::*;

/// Reveals text character by character out of a blur.
#[story(title = "Text/BlurText", tags = ["text"])]
pub fn blur_text(
    #[default = "Blur reveal"] text: String,
    #[default = 0.5] duration: f64,
    #[default = 0.02] stagger_delay: f64,
    #[default = "8px"] initial_blur: String,
) -> Element {
    rsx! {
        BlurText {
            text,
            duration,
            stagger_delay,
            initial_blur,
        }
    }
}

/// Lifts text into place one character at a time.
#[story(title = "Text/CharacterStagger", tags = ["text"])]
pub fn character_stagger(
    #[default = "Character stagger"] text: String,
    #[default = 0.4] duration: f64,
    #[default = 0.015] stagger_delay: f64,
    #[default = 15.0] y_offset: f64,
) -> Element {
    rsx! {
        CharacterStagger {
            text,
            duration,
            stagger_delay,
            y_offset,
        }
    }
}

/// Reveals text one word at a time.
#[story(title = "Text/WordReveal", tags = ["text"])]
pub fn word_reveal(
    #[default = "One word at a time"] text: String,
    #[default = 0.5] duration: f64,
    #[default = 0.04] stagger_delay: f64,
) -> Element {
    rsx! {
        WordReveal {
            text,
            duration,
            stagger_delay,
        }
    }
}

/// Slides each line up from behind a mask.
#[story(title = "Text/TextReveal", tags = ["text"])]
pub fn text_reveal(
    #[default = "Line by line
from behind a mask"]
    text: String,
    #[default = 0.8] duration: f64,
    #[default = 0.15] stagger_delay: f64,
) -> Element {
    rsx! {
        TextReveal {
            text,
            duration,
            stagger_delay,
        }
    }
}
