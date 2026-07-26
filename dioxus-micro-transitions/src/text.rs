//! Staggered text reveals.
//!
//! These split their `text` into characters, words or lines and stagger each
//! piece. They animate on mount rather than on intersection, so if you need
//! them to wait for the viewport, wrap them in
//! [`ScrollReveal`](crate::scroll::ScrollReveal) or mount them lazily.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_micro_transitions::text::WordReveal;
//!
//! fn Intro() -> Element {
//!     rsx! {
//!         WordReveal { text: "Motion, without the JavaScript." }
//!     }
//! }
//! ```

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[
    BLUR_TEXT_CSS,
    CHARACTER_STAGGER_CSS,
    WORD_REVEAL_CSS,
    TEXT_REVEAL_CSS,
];

const BLUR_TEXT_CSS: &str = r#"
@keyframes amt-blur-text{from{opacity:0;filter:blur(var(--amt-blur))}to{opacity:1;filter:blur(0)}}
.amt-blur-text{display:inline-block}
.amt-blur-text span{display:inline-block;white-space:pre;animation:amt-blur-text var(--amt-duration) ease-out both;animation-delay:var(--amt-delay)}
@media (prefers-reduced-motion:reduce){.amt-blur-text span{animation:none}}
"#;

/// Reveals text character by character, each pulling out of a blur.
#[component]
pub fn BlurText(
    /// The text to reveal.
    text: String,
    /// Length of one character's reveal, in seconds.
    #[props(default = 0.5)]
    duration: f64,
    /// Delay added per character, in seconds.
    #[props(default = 0.02)]
    stagger_delay: f64,
    /// Blur radius each character starts at, as a CSS length.
    #[props(default = "8px".to_string())]
    initial_blur: String,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("blur-text", BLUR_TEXT_CSS)}
        div {
            class: "amt amt-blur-text {class}",
            style: "--amt-duration:{duration}s;--amt-blur:{initial_blur};",
            aria_label: "{text}",
            ..attributes,
            for (i , ch) in text.chars().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "--amt-delay:{i as f64 * stagger_delay}s;",
                    "{ch}"
                }
            }
        }
    }
}

const CHARACTER_STAGGER_CSS: &str = r#"
@keyframes amt-character-stagger{from{opacity:0;transform:translateY(var(--amt-offset)) scale(.8)}to{opacity:1;transform:none}}
.amt-character-stagger{display:inline-block}
.amt-character-stagger span{display:inline-block;white-space:pre;animation:amt-character-stagger var(--amt-duration) cubic-bezier(.34,1.56,.64,1) both;animation-delay:var(--amt-delay)}
@media (prefers-reduced-motion:reduce){.amt-character-stagger span{animation:none}}
"#;

/// Pops text in one character at a time, each overshooting slightly.
///
/// The upstream component uses a spring; this port approximates it with the
/// equivalent overshooting cubic-bezier.
#[component]
pub fn CharacterStagger(
    /// The text to reveal.
    text: String,
    /// Length of one character's reveal, in seconds.
    #[props(default = 0.4)]
    duration: f64,
    /// Delay added per character, in seconds.
    #[props(default = 0.015)]
    stagger_delay: f64,
    /// How far below its final position each character starts, in pixels.
    #[props(default = 15.0)]
    y_offset: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("character-stagger", CHARACTER_STAGGER_CSS)}
        div {
            class: "amt amt-character-stagger {class}",
            style: "--amt-duration:{duration}s;--amt-offset:{y_offset}px;",
            aria_label: "{text}",
            ..attributes,
            for (i , ch) in text.chars().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "--amt-delay:{i as f64 * stagger_delay}s;",
                    "{ch}"
                }
            }
        }
    }
}

const WORD_REVEAL_CSS: &str = r#"
@keyframes amt-word-reveal{from{opacity:0;transform:translateY(15px) scale(.9)}to{opacity:1;transform:none}}
.amt-word-reveal{display:flex;flex-wrap:wrap;column-gap:.5rem;row-gap:.375rem}
.amt-word-reveal span{display:inline-block;animation:amt-word-reveal var(--amt-duration) cubic-bezier(.215,.61,.355,1) both;animation-delay:var(--amt-delay)}
@media (prefers-reduced-motion:reduce){.amt-word-reveal span{animation:none}}
"#;

/// Reveals text one word at a time, each rising and settling.
#[component]
pub fn WordReveal(
    /// The text to reveal. Split on whitespace.
    text: String,
    /// Length of one word's reveal, in seconds.
    #[props(default = 0.5)]
    duration: f64,
    /// Delay added per word, in seconds.
    #[props(default = 0.04)]
    stagger_delay: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("word-reveal", WORD_REVEAL_CSS)}
        div {
            class: "amt amt-word-reveal {class}",
            style: "--amt-duration:{duration}s;",
            aria_label: "{text}",
            ..attributes,
            for (i , word) in text.split_whitespace().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "--amt-delay:{i as f64 * stagger_delay}s;",
                    "{word}"
                }
            }
        }
    }
}

const TEXT_REVEAL_CSS: &str = r#"
@keyframes amt-text-reveal{from{transform:translateY(100%)}to{transform:none}}
.amt-text-reveal{display:flex;flex-direction:column}
.amt-text-reveal div{overflow:hidden;padding:.25rem 0}
.amt-text-reveal span{display:block;animation:amt-text-reveal var(--amt-duration) cubic-bezier(.16,1,.3,1) both;animation-delay:var(--amt-delay)}
@media (prefers-reduced-motion:reduce){.amt-text-reveal span{animation:none}}
"#;

/// Reveals text line by line, each sliding up from behind a mask.
#[component]
pub fn TextReveal(
    /// The text to reveal. Split on newlines; each line gets its own mask.
    text: String,
    /// Length of one line's reveal, in seconds.
    #[props(default = 0.8)]
    duration: f64,
    /// Delay added per line, in seconds.
    #[props(default = 0.15)]
    stagger_delay: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("text-reveal", TEXT_REVEAL_CSS)}
        div {
            class: "amt amt-text-reveal {class}",
            style: "--amt-duration:{duration}s;",
            aria_label: "{text}",
            ..attributes,
            for (i , line) in text.lines().enumerate() {
                div { key: "{i}", aria_hidden: "true",
                    span { style: "--amt-delay:{i as f64 * stagger_delay}s;", "{line}" }
                }
            }
        }
    }
}
