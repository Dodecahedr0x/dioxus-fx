use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-sound-wave{display:flex;align-items:center;gap:calc(var(--dfx-size)*.125);height:var(--dfx-size)}
.dfx-apple-sound-wave span{width:calc(var(--dfx-size)*.125);height:calc(var(--dfx-size)*.125);border-radius:9999px;background:var(--dfx-color);animation:dfx-apple-sound-wave var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-apple-sound-wave{0%,100%{height:calc(var(--dfx-size)*.125)}50%{height:var(--dfx-peak)}}
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
        {dfx_style!("apple-sound-wave", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-sound-wave {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for (i , peak) in PEAKS.iter().enumerate() {
                span {
                    key: "{i}",
                    style: "--dfx-peak:{peak * size}px;animation-delay:{i as f64 * 0.1}s;",
                }
            }
        }
    }
}
