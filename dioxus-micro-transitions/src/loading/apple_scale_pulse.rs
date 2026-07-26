use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-apple-scale-pulse{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-apple-scale-pulse i{position:absolute;inset:0;border-radius:9999px;background:var(--amt-track);animation:amt-apple-scale-pulse var(--amt-duration) ease-out infinite}
.amt-apple-scale-pulse b{position:relative;width:calc(var(--amt-size)*.25);height:calc(var(--amt-size)*.25);border-radius:9999px;background:var(--amt-color)}
@keyframes amt-apple-scale-pulse{from{transform:scale(0);opacity:1}to{transform:scale(1);opacity:0}}
"#;

/// A solid centre with two rings expanding out of it and fading.
#[component]
pub fn AppleScalePulse(
    /// Width and height of the loader, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Centre-dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one ring's expansion, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("apple-scale-pulse", CSS)}
        div {
            class: "amt amt-loader amt-apple-scale-pulse {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            i { style: "animation-delay:{duration / 3.0}s;" }
            b {}
        }
    }
}
