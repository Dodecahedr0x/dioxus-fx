use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-elastic-square{display:flex;align-items:flex-end;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border-bottom:2px solid var(--dfx-track)}
.dfx-elastic-square span{width:calc(var(--dfx-size)*.5);height:calc(var(--dfx-size)*.5);border-radius:3px;background:var(--dfx-color);transform-origin:bottom center;animation:dfx-elastic-square var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-elastic-square{0%,100%{transform:translateY(0) scale(1.3,.7)}50%{transform:translateY(calc(var(--dfx-size)*-.417)) scale(.8,1.2)}}
"#;

/// A squishy square bouncing off a baseline.
#[component]
pub fn ElasticSquare(
    /// Width and height of the frame, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Square colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("elastic-square", CSS)}
        div {
            class: "dfx dfx-loader dfx-elastic-square {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
