use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bounce-dots{display:flex;align-items:center;gap:calc(var(--dfx-size)*.6)}
.dfx-bounce-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-bounce-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bounce-dots{0%,100%{transform:translateY(0)}50%{transform:translateY(calc(var(--dfx-size)*-.8))}}
"#;

/// Three small dots hopping in quick succession.
#[component]
pub fn BounceDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 10.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one hop, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("bounce-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-bounce-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
