use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-symmetric-wave{display:flex;align-items:center;gap:6px;height:var(--dfx-size)}
.dfx-symmetric-wave span{width:calc(var(--dfx-size)*.15);border-radius:9999px;background:var(--dfx-color);animation:dfx-symmetric-wave var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-symmetric-wave{0%,100%{height:calc(var(--dfx-size)*.2)}50%{height:calc(var(--dfx-size)*.6)}}
"#;

/// Nine bars whose delays mirror around the centre, giving a symmetric ripple.
#[component]
pub fn SymmetricWave(
    /// Height of the wave, in pixels.
    #[props(default = 40.0)]
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
    // Delay steps mirrored about the middle bar.
    const STEPS: [i32; 9] = [0, 1, 2, 3, 4, 3, 2, 1, 0];
    rsx! {
        {dfx_style!("symmetric-wave", CSS)}
        div {
            class: "dfx dfx-loader dfx-symmetric-wave {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for (i , step) in STEPS.iter().enumerate() {
                span { key: "{i}", style: "animation-delay:{*step as f64 * 0.1}s;" }
            }
        }
    }
}
