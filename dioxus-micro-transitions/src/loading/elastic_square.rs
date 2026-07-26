use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-elastic-square{display:flex;align-items:flex-end;justify-content:center;width:var(--amt-size);height:var(--amt-size);border-bottom:2px solid var(--amt-track)}
.amt-elastic-square span{width:calc(var(--amt-size)*.5);height:calc(var(--amt-size)*.5);border-radius:3px;background:var(--amt-color);transform-origin:bottom center;animation:amt-elastic-square var(--amt-duration) ease-in-out infinite}
@keyframes amt-elastic-square{0%,100%{transform:translateY(0) scale(1.3,.7)}50%{transform:translateY(calc(var(--amt-size)*-.417)) scale(.8,1.2)}}
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
        {amt_style!("elastic-square", CSS)}
        div {
            class: "amt amt-loader amt-elastic-square {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
