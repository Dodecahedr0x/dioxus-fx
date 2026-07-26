use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-square-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:calc(var(--amt-size)*.375)}
.amt-square-grid span{width:var(--amt-size);height:var(--amt-size);border-radius:3px;background:var(--amt-color);animation:amt-square-grid var(--amt-duration) ease-in-out infinite}
@keyframes amt-square-grid{0%,100%{transform:scale(1);opacity:1}50%{transform:scale(.5);opacity:.3}}
"#;

/// Four squares blinking clockwise around a two-by-two grid.
#[component]
pub fn SquareGrid(
    /// Width and height of one square, in pixels.
    #[props(default = 16.0)]
    size: f64,
    /// Square colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("square-grid", CSS)}
        div {
            class: "amt amt-loader amt-square-grid {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
