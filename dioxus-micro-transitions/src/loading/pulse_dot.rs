use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-pulse-dot{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-pulse-dot i{position:absolute;inset:0;border-radius:9999px;background:var(--amt-color);animation:amt-pulse-dot var(--amt-duration) ease-out infinite}
.amt-pulse-dot b{position:relative;width:calc(var(--amt-size)*.3);height:calc(var(--amt-size)*.3);border-radius:9999px;background:var(--amt-color)}
@keyframes amt-pulse-dot{from{transform:scale(0);opacity:.8}to{transform:scale(1);opacity:0}}
"#;

/// A dot with a single halo growing out of it and dissolving.
#[component]
pub fn PulseDot(
    /// Diameter of the halo, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the halo to expand and fade, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("pulse-dot", CSS)}
        div {
            class: "amt amt-loader amt-pulse-dot {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            b {}
        }
    }
}
