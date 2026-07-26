use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-square-snake{display:grid;grid-template-columns:repeat(3,1fr);gap:4px;width:var(--amt-size);height:var(--amt-size)}
.amt-square-snake span{width:100%;height:100%;border-radius:3px;background:var(--amt-color);animation:amt-square-snake var(--amt-duration) ease-in-out infinite}
@keyframes amt-square-snake{0%,100%{opacity:.1}50%{opacity:1}}
"#;

/// A nine-tile block lighting up along a diagonal.
#[component]
pub fn SquareSnake(
    /// Width and height of the block, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Tile colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("square-snake", CSS)}
        div {
            class: "amt amt-loader amt-square-snake {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..9 {
                span {
                    key: "{i}",
                    style: "animation-delay:{((i % 3) + (i / 3)) as f64 * 0.15}s;",
                }
            }
        }
    }
}
