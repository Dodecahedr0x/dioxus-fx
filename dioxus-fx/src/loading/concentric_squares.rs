use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-concentric-squares{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-concentric-squares span{position:absolute;border:2px solid var(--dfx-color);border-radius:3px;animation:var(--dfx-duration) ease-in-out infinite}
.dfx-concentric-squares span:nth-child(1){width:100%;height:100%;animation-name:dfx-concentric-squares-out}
.dfx-concentric-squares span:nth-child(2){width:50%;height:50%;opacity:.6;animation-name:dfx-concentric-squares-in}
@keyframes dfx-concentric-squares-out{0%{transform:rotate(0) scale(1)}50%{transform:rotate(90deg) scale(1)}75%{transform:rotate(90deg) scale(.8)}100%{transform:rotate(90deg) scale(1)}}
@keyframes dfx-concentric-squares-in{0%{transform:rotate(0) scale(1)}50%{transform:rotate(-90deg) scale(1)}75%{transform:rotate(-90deg) scale(1.2)}100%{transform:rotate(-90deg) scale(1)}}
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
        {dfx_style!("concentric-squares", CSS)}
        div {
            class: "dfx dfx-loader dfx-concentric-squares {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
