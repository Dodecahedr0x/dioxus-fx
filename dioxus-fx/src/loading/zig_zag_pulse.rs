use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-zig-zag-pulse{display:grid;grid-template-columns:repeat(3,1fr);grid-template-rows:repeat(3,1fr);gap:8px;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-zig-zag-pulse span{width:calc(var(--dfx-size)*.208);height:calc(var(--dfx-size)*.208);border-radius:9999px;background:var(--dfx-color);animation:dfx-zig-zag-pulse var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-zig-zag-pulse{0%,100%{opacity:.2}50%{opacity:1}}
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
        {dfx_style!("zig-zag-pulse", CSS)}
        div {
            class: "dfx dfx-loader dfx-zig-zag-pulse {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
