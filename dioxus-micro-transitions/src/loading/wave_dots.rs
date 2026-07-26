use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-wave-dots{display:flex;align-items:center;gap:6px;height:calc(var(--amt-size)*3)}
.amt-wave-dots span{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);animation:amt-wave-dots var(--amt-duration) ease-in-out infinite}
@keyframes amt-wave-dots{0%,100%{transform:translateY(4px)}50%{transform:translateY(-4px)}}
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
        {amt_style!("wave-dots", CSS)}
        div {
            class: "amt amt-loader amt-wave-dots {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
