use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-zig-zag-pulse{display:grid;grid-template-columns:repeat(3,1fr);grid-template-rows:repeat(3,1fr);gap:8px;width:var(--amt-size);height:var(--amt-size)}
.amt-zig-zag-pulse span{width:calc(var(--amt-size)*.208);height:calc(var(--amt-size)*.208);border-radius:9999px;background:var(--amt-color);animation:amt-zig-zag-pulse var(--amt-duration) ease-in-out infinite}
@keyframes amt-zig-zag-pulse{0%,100%{opacity:.2}50%{opacity:1}}
"#;

/// Six dots placed on a zig-zag, lighting up along the path.
#[component]
pub fn ZigZagPulse(
    /// Width and height of the grid, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    // (column, row) pairs, 1-indexed for CSS grid placement.
    const CELLS: [(u8, u8); 6] = [(1, 1), (2, 2), (3, 1), (1, 3), (2, 2), (3, 3)];
    rsx! {
        {amt_style!("zig-zag-pulse", CSS)}
        div {
            class: "amt amt-loader amt-zig-zag-pulse {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for (i , (col , row)) in CELLS.iter().enumerate() {
                span {
                    key: "{i}",
                    style: "grid-column:{col};grid-row:{row};animation-delay:{i as f64 * 0.1}s;",
                }
            }
        }
    }
}
