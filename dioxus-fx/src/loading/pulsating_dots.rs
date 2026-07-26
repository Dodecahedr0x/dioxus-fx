use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-pulsating-dots{display:flex;align-items:center;gap:calc(var(--dfx-size)*.667)}
.dfx-pulsating-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-pulsating-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-pulsating-dots{0%,100%{transform:scale(1);opacity:.5}50%{transform:scale(1.5);opacity:1}}
"#;

/// Three dots swelling past their resting size in a rolling wave.
#[component]
pub fn PulsatingDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 12.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("pulsating-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-pulsating-dots {class}",
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
