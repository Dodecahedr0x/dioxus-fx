use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-waveform-loader{display:flex;align-items:center;gap:2px;height:var(--dfx-size)}
.dfx-waveform-loader span{width:calc(var(--dfx-size)*.125);border-radius:9999px;background:var(--dfx-color);animation:dfx-waveform-loader var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-waveform-loader{0%,100%{height:calc(var(--dfx-size)*.125)}50%{height:calc(var(--dfx-size)*.75)}}
"#;

/// Eight bars whose offsets follow a sine curve, giving an irregular waveform.
#[component]
pub fn WaveformLoader(
    /// Height of the waveform, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("waveform-loader", CSS)}
        div {
            class: "dfx dfx-loader dfx-waveform-loader {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..8 {
                span {
                    key: "{i}",
                    style: "animation-delay:{(i as f64).sin() * 0.5}s;",
                }
            }
        }
    }
}
