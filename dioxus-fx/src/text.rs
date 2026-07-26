//! Staggered text reveals.
//!
//! These split their `text` into characters, words or lines and stagger each
//! piece. They animate on mount rather than on intersection, so if you need
//! them to wait for the viewport, wrap them in
//! [`ScrollReveal`](crate::scroll::ScrollReveal) or mount them lazily.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_fx::text::WordReveal;
//!
//! fn Intro() -> Element {
//!     rsx! {
//!         WordReveal { text: "Motion, without the JavaScript." }
//!     }
//! }
//! ```

use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[
    BLUR_TEXT_CSS,
    CHARACTER_STAGGER_CSS,
    WORD_REVEAL_CSS,
    TEXT_REVEAL_CSS,
];

const BLUR_TEXT_CSS: &str = r#"
@keyframes dfx-blur-text{from{opacity:0;filter:blur(var(--dfx-blur))}to{opacity:1;filter:blur(0)}}
.dfx-blur-text{display:inline-block}
.dfx-blur-text span{display:inline-block;white-space:pre;animation:dfx-blur-text var(--dfx-duration) ease-out both;animation-delay:var(--dfx-delay)}
@media (prefers-reduced-motion:reduce){.dfx-blur-text span{animation:none}}
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
        {dfx_style!("blur-text", BLUR_TEXT_CSS)}
        div {
            class: "dfx dfx-blur-text {class}",
            style: "--dfx-duration:{duration}s;--dfx-blur:{initial_blur};",
            aria_label: "{text}",
            ..attributes,
            for (i , ch) in text.chars().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "--dfx-delay:{i as f64 * stagger_delay}s;",
                    "{ch}"
                }
            }
        }
    }
}

const CHARACTER_STAGGER_CSS: &str = r#"
@keyframes dfx-character-stagger{from{opacity:0;transform:translateY(var(--dfx-offset)) scale(.8)}to{opacity:1;transform:none}}
.dfx-character-stagger{display:inline-block}
.dfx-character-stagger span{display:inline-block;white-space:pre;animation:dfx-character-stagger var(--dfx-duration) cubic-bezier(.34,1.56,.64,1) both;animation-delay:var(--dfx-delay)}
@media (prefers-reduced-motion:reduce){.dfx-character-stagger span{animation:none}}
"#;

/// Pops text in one character at a time, each overshooting slightly.
///
/// The overshoot is an overshooting cubic-bezier rather than a simulated
/// spring, which reads the same at this duration and costs no frames.
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
        {dfx_style!("character-stagger", CHARACTER_STAGGER_CSS)}
        div {
            class: "dfx dfx-character-stagger {class}",
            style: "--dfx-duration:{duration}s;--dfx-offset:{y_offset}px;",
            aria_label: "{text}",
            ..attributes,
            for (i , ch) in text.chars().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "--dfx-delay:{i as f64 * stagger_delay}s;",
                    "{ch}"
                }
            }
        }
    }
}

const WORD_REVEAL_CSS: &str = r#"
@keyframes dfx-word-reveal{from{opacity:0;transform:translateY(15px) scale(.9)}to{opacity:1;transform:none}}
.dfx-word-reveal{display:flex;flex-wrap:wrap;column-gap:.5rem;row-gap:.375rem}
.dfx-word-reveal span{display:inline-block;animation:dfx-word-reveal var(--dfx-duration) cubic-bezier(.215,.61,.355,1) both;animation-delay:var(--dfx-delay)}
@media (prefers-reduced-motion:reduce){.dfx-word-reveal span{animation:none}}
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
        {dfx_style!("word-reveal", WORD_REVEAL_CSS)}
        div {
            class: "dfx dfx-word-reveal {class}",
            style: "--dfx-duration:{duration}s;",
            aria_label: "{text}",
            ..attributes,
            for (i , word) in text.split_whitespace().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "--dfx-delay:{i as f64 * stagger_delay}s;",
                    "{word}"
                }
            }
        }
    }
}

const TEXT_REVEAL_CSS: &str = r#"
@keyframes dfx-text-reveal{from{transform:translateY(100%)}to{transform:none}}
.dfx-text-reveal{display:flex;flex-direction:column}
.dfx-text-reveal div{overflow:hidden;padding:.25rem 0}
.dfx-text-reveal span{display:block;animation:dfx-text-reveal var(--dfx-duration) cubic-bezier(.16,1,.3,1) both;animation-delay:var(--dfx-delay)}
@media (prefers-reduced-motion:reduce){.dfx-text-reveal span{animation:none}}
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
        {dfx_style!("text-reveal", TEXT_REVEAL_CSS)}
        div {
            class: "dfx dfx-text-reveal {class}",
            style: "--dfx-duration:{duration}s;",
            aria_label: "{text}",
            ..attributes,
            for (i , line) in text.lines().enumerate() {
                div { key: "{i}", aria_hidden: "true",
                    span { style: "--dfx-delay:{i as f64 * stagger_delay}s;", "{line}" }
                }
            }
        }
    }
}
