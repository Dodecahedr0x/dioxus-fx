use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-pulse-dots{display:flex;align-items:center;gap:calc(var(--dfx-size)*.6)}
.dfx-pulse-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-pulse-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-pulse-dots{0%,100%{opacity:.2}50%{opacity:1}}
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
        {dfx_style!("pulse-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-pulse-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
