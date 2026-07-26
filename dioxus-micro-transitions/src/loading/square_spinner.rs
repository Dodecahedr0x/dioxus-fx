use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-square-spinner{display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);border:2px solid var(--amt-color);animation:amt-square-spinner var(--amt-duration) ease-in-out infinite}
.amt-square-spinner span{width:25%;height:25%;background:var(--amt-color)}
@keyframes amt-square-spinner{from{transform:rotate(0)}to{transform:rotate(90deg)}}
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
        {amt_style!("square-spinner", CSS)}
        div {
            class: "amt amt-loader amt-square-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
