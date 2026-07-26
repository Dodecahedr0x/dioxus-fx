use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-siri-wave{display:flex;align-items:center;gap:calc(var(--dfx-size)*.125);height:var(--dfx-size)}
.dfx-siri-wave span{width:calc(var(--dfx-size)*.125);border-radius:9999px;background:var(--dfx-color);animation:dfx-siri-wave var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-siri-wave{0%,100%{height:calc(var(--dfx-size)*.125)}50%{height:calc(var(--dfx-size)*.75)}}
"#;

/// Five bars rippling upward in sequence, like a voice assistant listening.
#[component]
pub fn SiriWave(
    /// Height of the wave, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("siri-wave", CSS)}
        div {
            class: "dfx dfx-loader dfx-siri-wave {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
