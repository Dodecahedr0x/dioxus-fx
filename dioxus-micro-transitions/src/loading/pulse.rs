use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-pulse{position:relative;display:inline-flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-pulse i{position:absolute;inset:0;border-radius:9999px;background:var(--amt-color)}
.amt-pulse span{position:absolute;inset:0;border-radius:9999px;background:var(--amt-color);opacity:.4;animation:amt-pulse var(--amt-duration) ease-out infinite}
@keyframes amt-pulse{from{transform:scale(1);opacity:.5}to{transform:scale(2.8);opacity:0}}
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
        {amt_style!("pulse", CSS)}
        div {
            class: "amt amt-loader amt-pulse {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            span {}
            span { style: "animation-delay:{duration / 3.0}s;" }
        }
    }
}
