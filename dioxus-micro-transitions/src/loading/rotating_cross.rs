use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-rotating-cross{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);animation:amt-rotating-cross var(--amt-duration) ease-in-out infinite}
.amt-rotating-cross span{position:absolute;background:var(--amt-color);border-radius:3px}
.amt-rotating-cross span:nth-child(1){width:100%;height:calc(var(--amt-size)*.1875)}
.amt-rotating-cross span:nth-child(2){height:100%;width:calc(var(--amt-size)*.1875)}
@keyframes amt-rotating-cross{from{transform:rotate(0)}to{transform:rotate(180deg)}}
"#;

/// A plus sign flipping through a half turn, over and over.
#[component]
pub fn RotatingCross(
    /// Width and height of the cross, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one half turn, in seconds.
    #[props(default = 0.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("rotating-cross", CSS)}
        div {
            class: "amt amt-loader amt-rotating-cross {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
