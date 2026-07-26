use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-scale-pulse{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-apple-scale-pulse i{position:absolute;inset:0;border-radius:9999px;background:var(--dfx-track);animation:dfx-apple-scale-pulse var(--dfx-duration) ease-out infinite}
.dfx-apple-scale-pulse b{position:relative;width:calc(var(--dfx-size)*.25);height:calc(var(--dfx-size)*.25);border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-apple-scale-pulse{from{transform:scale(0);opacity:1}to{transform:scale(1);opacity:0}}
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
        {dfx_style!("apple-scale-pulse", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-scale-pulse {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            i { style: "animation-delay:{duration / 3.0}s;" }
            b {}
        }
    }
}
