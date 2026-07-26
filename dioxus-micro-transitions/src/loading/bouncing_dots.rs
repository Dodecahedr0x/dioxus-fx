use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-bouncing-dots{position:relative;display:flex;align-items:flex-end;justify-content:center;width:var(--amt-size);height:calc(var(--amt-size)*.75);padding-bottom:8px;border-bottom:2px solid var(--amt-track)}
.amt-bouncing-dots span{position:absolute;bottom:8px;width:calc(var(--amt-size)*.1875);height:calc(var(--amt-size)*.1875);border-radius:9999px;background:var(--amt-color);transform-origin:bottom center;animation:amt-bouncing-dots var(--amt-duration) ease-in-out infinite}
@keyframes amt-bouncing-dots{0%,100%{transform:translateY(0) scaleY(.8)}50%{transform:translateY(calc(var(--amt-size)*-.3125)) scaleY(1.1)}}
"#;

/// Three dots bouncing off a baseline, stretching as they leave it.
#[component]
pub fn BouncingDots(
    /// Width of the frame, in pixels.
    #[props(default = 64.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one bounce, in seconds.
    #[props(default = 0.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("bouncing-dots", CSS)}
        div {
            class: "amt amt-loader amt-bouncing-dots {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span {
                    key: "{i}",
                    style: "left:{(i as f64 * 14.0 + 10.0) * size / 64.0}px;animation-delay:{i as f64 * 0.1}s;",
                }
            }
        }
    }
}
