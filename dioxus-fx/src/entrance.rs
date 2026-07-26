//! Mount-time entrance animations.
//!
//! Each component wraps its children in a `div` that animates once, as soon as
//! it is inserted into the DOM. They are drop-in wrappers: give them `duration`
//! and `delay` in seconds and stagger them by hand for list intros.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_fx::entrance::FadeUp;
//!
//! fn Hero() -> Element {
//!     rsx! {
//!         FadeUp { delay: 0.1, h1 { "Ship it" } }
//!     }
//! }
//! ```

use crate::style::dfx_style;
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
@keyframes dfx-fade-in{from{opacity:0}to{opacity:1}}
.dfx-fade-in{animation:dfx-fade-in var(--dfx-duration) cubic-bezier(.215,.61,.355,1) var(--dfx-delay) both}
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
        {dfx_style!("fade-in", FADE_IN_CSS)}
        div {
            class: "dfx dfx-decorative dfx-fade-in {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;",
            ..attributes,
            {children}
        }
    }
}

const FADE_UP_CSS: &str = r#"
@keyframes dfx-fade-up{from{opacity:0;transform:translateY(var(--dfx-offset))}to{opacity:1;transform:none}}
.dfx-fade-up{animation:dfx-fade-up var(--dfx-duration) cubic-bezier(.16,1,.3,1) var(--dfx-delay) both}
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
        {dfx_style!("fade-up", FADE_UP_CSS)}
        div {
            class: "dfx dfx-decorative dfx-fade-up {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-offset:{y_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const FADE_DOWN_CSS: &str = r#"
@keyframes dfx-fade-down{from{opacity:0;transform:translateY(var(--dfx-offset))}to{opacity:1;transform:none}}
.dfx-fade-down{animation:dfx-fade-down var(--dfx-duration) cubic-bezier(.16,1,.3,1) var(--dfx-delay) both}
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
        {dfx_style!("fade-down", FADE_DOWN_CSS)}
        div {
            class: "dfx dfx-decorative dfx-fade-down {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-offset:{y_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const SLIDE_LEFT_CSS: &str = r#"
@keyframes dfx-slide-left{from{opacity:0;transform:translateX(var(--dfx-offset))}to{opacity:1;transform:none}}
.dfx-slide-left{animation:dfx-slide-left var(--dfx-duration) cubic-bezier(.16,1,.3,1) var(--dfx-delay) both}
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
        {dfx_style!("slide-left", SLIDE_LEFT_CSS)}
        div {
            class: "dfx dfx-decorative dfx-slide-left {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-offset:{x_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const SLIDE_RIGHT_CSS: &str = r#"
@keyframes dfx-slide-right{from{opacity:0;transform:translateX(var(--dfx-offset))}to{opacity:1;transform:none}}
.dfx-slide-right{animation:dfx-slide-right var(--dfx-duration) cubic-bezier(.16,1,.3,1) var(--dfx-delay) both}
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
        {dfx_style!("slide-right", SLIDE_RIGHT_CSS)}
        div {
            class: "dfx dfx-decorative dfx-slide-right {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-offset:{x_offset}px;",
            ..attributes,
            {children}
        }
    }
}

const SCALE_IN_CSS: &str = r#"
@keyframes dfx-scale-in{from{opacity:0;transform:scale(var(--dfx-scale))}to{opacity:1;transform:none}}
.dfx-scale-in{animation:dfx-scale-in var(--dfx-duration) cubic-bezier(.34,1.56,.64,1) var(--dfx-delay) both}
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
        {dfx_style!("scale-in", SCALE_IN_CSS)}
        div {
            class: "dfx dfx-decorative dfx-scale-in {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-scale:{initial_scale};",
            ..attributes,
            {children}
        }
    }
}

const ZOOM_IN_CSS: &str = r#"
@keyframes dfx-zoom-in{from{opacity:0;transform:scale(var(--dfx-scale));filter:blur(var(--dfx-blur))}to{opacity:1;transform:none;filter:blur(0)}}
.dfx-zoom-in{animation:dfx-zoom-in var(--dfx-duration) cubic-bezier(.16,1,.3,1) var(--dfx-delay) both}
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
        {dfx_style!("zoom-in", ZOOM_IN_CSS)}
        div {
            class: "dfx dfx-decorative dfx-zoom-in {class}",
            style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-scale:{initial_scale};--dfx-blur:{initial_blur};",
            ..attributes,
            {children}
        }
    }
}
