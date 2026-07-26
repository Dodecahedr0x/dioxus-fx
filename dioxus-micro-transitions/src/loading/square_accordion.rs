use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-square-accordion{display:flex;align-items:center;justify-content:center;gap:4px;width:var(--amt-size);height:var(--amt-size)}
.amt-square-accordion span{width:calc(var(--amt-size)*.25);border-radius:3px;background:var(--amt-color);animation:amt-square-accordion var(--amt-duration) ease-in-out infinite}
@keyframes amt-square-accordion{0%,100%{height:calc(var(--amt-size)*.25)}50%{height:calc(var(--amt-size)*.667)}}
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
        {amt_style!("square-accordion", CSS)}
        div {
            class: "amt amt-loader amt-square-accordion {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
