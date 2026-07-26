use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-siri-wave{display:flex;align-items:center;gap:calc(var(--amt-size)*.125);height:var(--amt-size)}
.amt-siri-wave span{width:calc(var(--amt-size)*.125);border-radius:9999px;background:var(--amt-color);animation:amt-siri-wave var(--amt-duration) ease-in-out infinite}
@keyframes amt-siri-wave{0%,100%{height:calc(var(--amt-size)*.125)}50%{height:calc(var(--amt-size)*.75)}}
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
        {amt_style!("siri-wave", CSS)}
        div {
            class: "amt amt-loader amt-siri-wave {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
