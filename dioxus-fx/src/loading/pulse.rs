use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-pulse{position:relative;display:inline-flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-pulse i{position:absolute;inset:0;border-radius:9999px;background:var(--dfx-color)}
.dfx-pulse span{position:absolute;inset:0;border-radius:9999px;background:var(--dfx-color);opacity:.4;animation:dfx-pulse var(--dfx-duration) ease-out infinite}
@keyframes dfx-pulse{from{transform:scale(1);opacity:.5}to{transform:scale(2.8);opacity:0}}
"#;

/// A status dot with two rings rippling outward from it.
#[component]
pub fn Pulse(
    /// Diameter of the centre dot, in pixels.
    #[props(default = 12.0)]
    size: f64,
    /// Dot colour. Any CSS colour.
    #[props(default = "#3b82f6".to_string())]
    color: String,
    /// Time for one ring to expand and fade, in seconds.
    #[props(default = 1.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("pulse", CSS)}
        div {
            class: "dfx dfx-loader dfx-pulse {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            span {}
            span { style: "animation-delay:{duration / 3.0}s;" }
        }
    }
}
