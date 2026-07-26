use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bouncing-square{position:relative;display:flex;align-items:flex-end;justify-content:center;width:calc(var(--dfx-size)*.833);height:var(--dfx-size)}
.dfx-bouncing-square i{position:absolute;bottom:0;width:calc(var(--dfx-size)*.667);height:calc(var(--dfx-size)*.083);border-radius:9999px;background:var(--dfx-track);filter:blur(2px)}
.dfx-bouncing-square span{width:calc(var(--dfx-size)*.5);height:calc(var(--dfx-size)*.5);border-radius:3px;background:var(--dfx-color);transform-origin:bottom center;animation:dfx-bouncing-square var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bouncing-square{0%,100%{transform:translateY(0) scale(1.2,.8)}50%{transform:translateY(calc(var(--dfx-size)*-.5)) scale(.9,1.1)}}
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
        {dfx_style!("bouncing-square", CSS)}
        div {
            class: "dfx dfx-loader dfx-bouncing-square {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            span {}
        }
    }
}
