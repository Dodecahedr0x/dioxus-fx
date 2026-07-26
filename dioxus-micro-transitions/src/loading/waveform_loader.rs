use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-waveform-loader{display:flex;align-items:center;gap:2px;height:var(--amt-size)}
.amt-waveform-loader span{width:calc(var(--amt-size)*.125);border-radius:9999px;background:var(--amt-color);animation:amt-waveform-loader var(--amt-duration) ease-in-out infinite}
@keyframes amt-waveform-loader{0%,100%{height:calc(var(--amt-size)*.125)}50%{height:calc(var(--amt-size)*.75)}}
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
        {amt_style!("waveform-loader", CSS)}
        div {
            class: "amt amt-loader amt-waveform-loader {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
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
