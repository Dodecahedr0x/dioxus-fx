use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-pulse-dots{display:flex;align-items:center;gap:calc(var(--amt-size)*.6)}
.amt-pulse-dots span{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);animation:amt-pulse-dots var(--amt-duration) ease-in-out infinite}
@keyframes amt-pulse-dots{0%,100%{opacity:.2}50%{opacity:1}}
"#;

/// Three dots brightening and dimming in a rolling sequence.
#[component]
pub fn PulseDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 10.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.4)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("pulse-dots", CSS)}
        div {
            class: "amt amt-loader amt-pulse-dots {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
