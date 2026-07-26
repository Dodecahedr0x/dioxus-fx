use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-fade-dots{display:flex;align-items:center;gap:calc(var(--dfx-size)*.8)}
.dfx-fade-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-fade-dots var(--dfx-duration) linear infinite}
@keyframes dfx-fade-dots{0%,100%{opacity:0}50%{opacity:1}}
"#;

/// Four dots fading up and out in a marching sequence.
#[component]
pub fn FadeDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 10.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("fade-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-fade-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
