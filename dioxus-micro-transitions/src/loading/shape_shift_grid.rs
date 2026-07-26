use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-shape-shift-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:calc(var(--amt-size)*.125);width:var(--amt-size);height:var(--amt-size)}
.amt-shape-shift-grid span{width:100%;height:100%;background:var(--amt-color);animation:amt-shape-shift-grid var(--amt-duration) ease-in-out infinite}
@keyframes amt-shape-shift-grid{0%,100%{border-radius:10%;transform:scale(1)}50%{border-radius:50%;transform:scale(.8)}}
"#;

/// A two-by-two block whose tiles round off into dots in sequence.
#[component]
pub fn ShapeShiftGrid(
    /// Width and height of the block, in pixels.
    #[props(default = 32.0)]
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
        {amt_style!("shape-shift-grid", CSS)}
        div {
            class: "amt amt-loader amt-shape-shift-grid {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
