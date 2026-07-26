use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-breathing-glow{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-breathing-glow i{position:absolute;width:66.6%;height:66.6%;border-radius:9999px;background:var(--amt-glow);filter:blur(8px);animation:amt-breathing-glow var(--amt-duration) ease-in-out infinite}
.amt-breathing-glow b{position:relative;width:50%;height:50%;border-radius:9999px;background:var(--amt-color)}
@keyframes amt-breathing-glow{0%,100%{transform:scale(1);opacity:.5}50%{transform:scale(1.5);opacity:.8}}
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
        {amt_style!("breathing-glow", CSS)}
        div {
            class: "amt amt-loader amt-breathing-glow {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-glow:{glow_color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            b {}
        }
    }
}
