use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-bouncing-square{position:relative;display:flex;align-items:flex-end;justify-content:center;width:calc(var(--amt-size)*.833);height:var(--amt-size)}
.amt-bouncing-square i{position:absolute;bottom:0;width:calc(var(--amt-size)*.667);height:calc(var(--amt-size)*.083);border-radius:9999px;background:var(--amt-track);filter:blur(2px)}
.amt-bouncing-square span{width:calc(var(--amt-size)*.5);height:calc(var(--amt-size)*.5);border-radius:3px;background:var(--amt-color);transform-origin:bottom center;animation:amt-bouncing-square var(--amt-duration) ease-in-out infinite}
@keyframes amt-bouncing-square{0%,100%{transform:translateY(0) scale(1.2,.8)}50%{transform:translateY(calc(var(--amt-size)*-.5)) scale(.9,1.1)}}
"#;

/// A square bouncing on its shadow, squashing on impact.
#[component]
pub fn BouncingSquare(
    /// Height of the frame, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Square colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one bounce, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("bouncing-square", CSS)}
        div {
            class: "amt amt-loader amt-bouncing-square {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            span {}
        }
    }
}
