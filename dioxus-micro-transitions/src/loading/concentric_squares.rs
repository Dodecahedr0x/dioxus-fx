use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-concentric-squares{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-concentric-squares span{position:absolute;border:2px solid var(--amt-color);border-radius:3px;animation:var(--amt-duration) ease-in-out infinite}
.amt-concentric-squares span:nth-child(1){width:100%;height:100%;animation-name:amt-concentric-squares-out}
.amt-concentric-squares span:nth-child(2){width:50%;height:50%;opacity:.6;animation-name:amt-concentric-squares-in}
@keyframes amt-concentric-squares-out{0%{transform:rotate(0) scale(1)}50%{transform:rotate(90deg) scale(1)}75%{transform:rotate(90deg) scale(.8)}100%{transform:rotate(90deg) scale(1)}}
@keyframes amt-concentric-squares-in{0%{transform:rotate(0) scale(1)}50%{transform:rotate(-90deg) scale(1)}75%{transform:rotate(-90deg) scale(1.2)}100%{transform:rotate(-90deg) scale(1)}}
"#;

/// Two nested squares turning against each other, one growing as the other shrinks.
#[component]
pub fn ConcentricSquares(
    /// Width and height of the outer square, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Outline colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("concentric-squares", CSS)}
        div {
            class: "amt amt-loader amt-concentric-squares {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
