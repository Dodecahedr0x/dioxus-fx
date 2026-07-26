use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-square-spinner{display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border:2px solid var(--dfx-color);animation:dfx-square-spinner var(--dfx-duration) ease-in-out infinite}
.dfx-square-spinner span{width:25%;height:25%;background:var(--dfx-color)}
@keyframes dfx-square-spinner{from{transform:rotate(0)}to{transform:rotate(90deg)}}
"#;

/// An outlined square stepping a quarter-turn at a time around a dot.
#[component]
pub fn SquareSpinner(
    /// Width and height of the square, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Outline and dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one quarter turn, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("square-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-square-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
