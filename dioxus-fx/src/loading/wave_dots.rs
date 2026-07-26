use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-wave-dots{display:flex;align-items:center;gap:6px;height:calc(var(--dfx-size)*3)}
.dfx-wave-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-wave-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-wave-dots{0%,100%{transform:translateY(4px)}50%{transform:translateY(-4px)}}
"#;

/// Five dots riding a sine wave from left to right.
#[component]
pub fn WaveDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 8.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("wave-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-wave-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
