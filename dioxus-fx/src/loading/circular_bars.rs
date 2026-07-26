use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-circular-bars{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-circular-bars span{position:absolute;top:0;left:calc(50% - var(--dfx-size)*.05);width:calc(var(--dfx-size)*.1);height:calc(var(--dfx-size)*.3);background:var(--dfx-color);border-radius:9999px;transform-origin:calc(var(--dfx-size)*.05) calc(var(--dfx-size)*.5);animation:dfx-circular-bars var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-circular-bars{0%,100%{transform:rotate(var(--dfx-rot)) scaleY(.5)}50%{transform:rotate(var(--dfx-rot)) scaleY(1.5)}}
"#;

/// Eight radial spokes that stretch and shrink around the dial in sequence.
#[component]
pub fn CircularBars(
    /// Width and height of the loader, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Spoke colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("circular-bars", CSS)}
        div {
            class: "dfx dfx-loader dfx-circular-bars {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..8 {
                span {
                    key: "{i}",
                    style: "--dfx-rot:{i * 45}deg;animation-delay:{i as f64 * 0.15}s;",
                }
            }
        }
    }
}
