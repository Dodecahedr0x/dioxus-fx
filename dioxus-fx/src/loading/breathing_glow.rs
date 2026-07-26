use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-breathing-glow{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-breathing-glow i{position:absolute;width:66.6%;height:66.6%;border-radius:9999px;background:var(--dfx-glow);filter:blur(8px);animation:dfx-breathing-glow var(--dfx-duration) ease-in-out infinite}
.dfx-breathing-glow b{position:relative;width:50%;height:50%;border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-breathing-glow{0%,100%{transform:scale(1);opacity:.5}50%{transform:scale(1.5);opacity:.8}}
"#;

/// A bright core sitting inside a halo that breathes in and out.
#[component]
pub fn BreathingGlow(
    /// Width and height of the loader, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Colour of the solid core.
    #[props(default = "#ffffff".to_string())]
    color: String,
    /// Colour of the surrounding glow.
    #[props(default = "#3b82f6".to_string())]
    glow_color: String,
    /// Length of one breath, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("breathing-glow", CSS)}
        div {
            class: "dfx dfx-loader dfx-breathing-glow {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-glow:{glow_color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            b {}
        }
    }
}
