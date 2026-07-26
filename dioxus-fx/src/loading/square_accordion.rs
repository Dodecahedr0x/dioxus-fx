use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-square-accordion{display:flex;align-items:center;justify-content:center;gap:4px;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-square-accordion span{width:calc(var(--dfx-size)*.25);border-radius:3px;background:var(--dfx-color);animation:dfx-square-accordion var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-square-accordion{0%,100%{height:calc(var(--dfx-size)*.25)}50%{height:calc(var(--dfx-size)*.667)}}
"#;

/// Three blocks stretching vertically in a rolling sequence.
#[component]
pub fn SquareAccordion(
    /// Width and height of the frame, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Block colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("square-accordion", CSS)}
        div {
            class: "dfx dfx-loader dfx-square-accordion {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
