use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-pulse-dots{display:flex;align-items:center;gap:calc(var(--dfx-size)*.8)}
.dfx-apple-pulse-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-apple-pulse-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-apple-pulse-dots{0%,100%{transform:scale(.5);opacity:.3}50%{transform:scale(1);opacity:1}}
"#;

/// Three dots that swell and fade in a rolling sequence.
#[component]
pub fn ApplePulseDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 10.0)]
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
        {dfx_style!("apple-pulse-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-pulse-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
