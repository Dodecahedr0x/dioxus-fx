use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bouncing-dots{position:relative;display:flex;align-items:flex-end;justify-content:center;width:var(--dfx-size);height:calc(var(--dfx-size)*.75);padding-bottom:8px;border-bottom:2px solid var(--dfx-track)}
.dfx-bouncing-dots span{position:absolute;bottom:8px;width:calc(var(--dfx-size)*.1875);height:calc(var(--dfx-size)*.1875);border-radius:9999px;background:var(--dfx-color);transform-origin:bottom center;animation:dfx-bouncing-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bouncing-dots{0%,100%{transform:translateY(0) scaleY(.8)}50%{transform:translateY(calc(var(--dfx-size)*-.3125)) scaleY(1.1)}}
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
        {dfx_style!("bouncing-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-bouncing-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
