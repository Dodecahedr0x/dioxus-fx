use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-rotating-cross{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);animation:dfx-rotating-cross var(--dfx-duration) ease-in-out infinite}
.dfx-rotating-cross span{position:absolute;background:var(--dfx-color);border-radius:3px}
.dfx-rotating-cross span:nth-child(1){width:100%;height:calc(var(--dfx-size)*.1875)}
.dfx-rotating-cross span:nth-child(2){height:100%;width:calc(var(--dfx-size)*.1875)}
@keyframes dfx-rotating-cross{from{transform:rotate(0)}to{transform:rotate(180deg)}}
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
        {dfx_style!("rotating-cross", CSS)}
        div {
            class: "dfx dfx-loader dfx-rotating-cross {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
