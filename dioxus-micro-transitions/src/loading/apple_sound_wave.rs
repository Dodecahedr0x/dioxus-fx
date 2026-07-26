use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-apple-sound-wave{display:flex;align-items:center;gap:calc(var(--amt-size)*.125);height:var(--amt-size)}
.amt-apple-sound-wave span{width:calc(var(--amt-size)*.125);height:calc(var(--amt-size)*.125);border-radius:9999px;background:var(--amt-color);animation:amt-apple-sound-wave var(--amt-duration) ease-in-out infinite}
@keyframes amt-apple-sound-wave{0%,100%{height:calc(var(--amt-size)*.125)}50%{height:var(--amt-peak)}}
"#;

/// Five bars forming a symmetric wave that swells from the centre outward.
#[component]
pub fn AppleSoundWave(
    /// Height of the wave, in pixels.
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
    // Peak heights as a fraction of `size`, mirroring the 1-2-3-2-1 profile.
    const PEAKS: [f64; 5] = [0.25, 0.5, 0.75, 0.5, 0.25];
    rsx! {
        {amt_style!("apple-sound-wave", CSS)}
        div {
            class: "amt amt-loader amt-apple-sound-wave {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for (i , peak) in PEAKS.iter().enumerate() {
                span {
                    key: "{i}",
                    style: "--amt-peak:{peak * size}px;animation-delay:{i as f64 * 0.1}s;",
                }
            }
        }
    }
}
