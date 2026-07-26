//! Mount-time entrance animations.
//!
//! Each component wraps its children in a `div` that animates once, as soon as
//! it is inserted into the DOM. They are drop-in wrappers: give them `duration`
//! and `delay` in seconds and stagger them by hand for list intros.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_micro_transitions::entrance::FadeUp;
//!
//! fn Hero() -> Element {
//!     rsx! {
//!         FadeUp { delay: 0.1, h1 { "Ship it" } }
//!     }
//! }
//! ```

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[
    FADE_IN_CSS,
    FADE_UP_CSS,
    FADE_DOWN_CSS,
    SLIDE_LEFT_CSS,
    SLIDE_RIGHT_CSS,
    SCALE_IN_CSS,
    ZOOM_IN_CSS,
];

const FADE_IN_CSS: &str = r#"
@keyframes amt-fade-in{from{opacity:0}to{opacity:1}}
.amt-fade-in{animation:amt-fade-in var(--amt-duration) cubic-bezier(.215,.61,.355,1) var(--amt-delay) both}
"#;

/// Fades its children in on mount.
///
/// Port of Amicro's `fade-in`; eases on `easeOutCubic`.
#[component]
pub fn FadeIn(
    /// Length of the animation, in seconds.
    #[props(default = 0.5)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("fade-in", FADE_IN_CSS)}
        div {
            class: "amt amt-decorative amt-fade-in {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;",
            ..attributes,
            {children}
        }
    }
}

const FADE_UP_CSS: &str = r#"
@keyframes amt-fade-up{from{opacity:0;transform:translateY(var(--amt-offset))}to{opacity:1;transform:none}}
.amt-fade-up{animation:amt-fade-up var(--amt-duration) cubic-bezier(.16,1,.3,1) var(--amt-delay) both}
"#;

/// Fades its children in while they rise into place.
///
/// Port of Amicro's `fade-up`; eases on `easeOutExpo`.
#[component]
pub fn FadeUp(
    /// Length of the animation, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// How far below its final position the content starts, in pixels.
    #[props(default = 20.0)]
    y_offset: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("fade-up", FADE_UP_CSS)}
        div {
            class: "amt amt-decorative amt-fade-up {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;--amt-offset:{y_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const FADE_DOWN_CSS: &str = r#"
@keyframes amt-fade-down{from{opacity:0;transform:translateY(var(--amt-offset))}to{opacity:1;transform:none}}
.amt-fade-down{animation:amt-fade-down var(--amt-duration) cubic-bezier(.16,1,.3,1) var(--amt-delay) both}
"#;

/// Fades its children in while they descend into place.
///
/// Port of Amicro's `fade-down`; eases on `easeOutExpo`.
#[component]
pub fn FadeDown(
    /// Length of the animation, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// How far above its final position the content starts, in pixels.
    #[props(default = -20.0)]
    y_offset: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("fade-down", FADE_DOWN_CSS)}
        div {
            class: "amt amt-decorative amt-fade-down {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;--amt-offset:{y_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const SLIDE_LEFT_CSS: &str = r#"
@keyframes amt-slide-left{from{opacity:0;transform:translateX(var(--amt-offset))}to{opacity:1;transform:none}}
.amt-slide-left{animation:amt-slide-left var(--amt-duration) cubic-bezier(.16,1,.3,1) var(--amt-delay) both}
"#;

/// Slides its children in from the right, settling leftward.
///
/// Port of Amicro's `slide-left`; eases on `easeOutExpo`.
#[component]
pub fn SlideLeft(
    /// Length of the animation, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// How far to the right the content starts, in pixels.
    #[props(default = 40.0)]
    x_offset: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("slide-left", SLIDE_LEFT_CSS)}
        div {
            class: "amt amt-decorative amt-slide-left {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;--amt-offset:{x_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const SLIDE_RIGHT_CSS: &str = r#"
@keyframes amt-slide-right{from{opacity:0;transform:translateX(var(--amt-offset))}to{opacity:1;transform:none}}
.amt-slide-right{animation:amt-slide-right var(--amt-duration) cubic-bezier(.16,1,.3,1) var(--amt-delay) both}
"#;

/// Slides its children in from the left, settling rightward.
///
/// Port of Amicro's `slide-right`; eases on `easeOutExpo`.
#[component]
pub fn SlideRight(
    /// Length of the animation, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// How far to the left the content starts, in pixels.
    #[props(default = -40.0)]
    x_offset: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("slide-right", SLIDE_RIGHT_CSS)}
        div {
            class: "amt amt-decorative amt-slide-right {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;--amt-offset:{x_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const SCALE_IN_CSS: &str = r#"
@keyframes amt-scale-in{from{opacity:0;transform:scale(var(--amt-scale))}to{opacity:1;transform:none}}
.amt-scale-in{animation:amt-scale-in var(--amt-duration) cubic-bezier(.34,1.56,.64,1) var(--amt-delay) both}
"#;

/// Pops its children in with a slight overshoot.
///
/// Port of Amicro's `scale-in`; the springy `cubic-bezier(.34,1.56,.64,1)`
/// curve overshoots past 1 before settling.
#[component]
pub fn ScaleIn(
    /// Length of the animation, in seconds.
    #[props(default = 0.5)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// Scale factor the content starts at.
    #[props(default = 0.92)]
    initial_scale: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("scale-in", SCALE_IN_CSS)}
        div {
            class: "amt amt-decorative amt-scale-in {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;--amt-scale:{initial_scale};",
            ..attributes,
            {children}
        }
    }
}

const ZOOM_IN_CSS: &str = r#"
@keyframes amt-zoom-in{from{opacity:0;transform:scale(var(--amt-scale));filter:blur(var(--amt-blur))}to{opacity:1;transform:none;filter:blur(0)}}
.amt-zoom-in{animation:amt-zoom-in var(--amt-duration) cubic-bezier(.16,1,.3,1) var(--amt-delay) both}
"#;

/// Zooms its children in while pulling them out of a blur.
///
/// Port of Amicro's `zoom-in`; eases on `easeOutExpo`.
#[component]
pub fn ZoomIn(
    /// Length of the animation, in seconds.
    #[props(default = 0.7)]
    duration: f64,
    /// How long to wait before starting, in seconds.
    #[props(default = 0.0)]
    delay: f64,
    /// Scale factor the content starts at.
    #[props(default = 0.85)]
    initial_scale: f64,
    /// Blur radius the content starts at, as a CSS length.
    #[props(default = "12px".to_string())]
    initial_blur: String,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        {amt_style!("zoom-in", ZOOM_IN_CSS)}
        div {
            class: "amt amt-decorative amt-zoom-in {class}",
            style: "--amt-duration:{duration}s;--amt-delay:{delay}s;--amt-scale:{initial_scale};--amt-blur:{initial_blur};",
            ..attributes,
            {children}
        }
    }
}
