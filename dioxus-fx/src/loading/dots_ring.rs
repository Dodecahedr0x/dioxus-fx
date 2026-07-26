use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-dots-ring{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-dots-ring span{position:absolute;top:0;left:calc(50% - var(--dfx-size)*.0833);width:calc(var(--dfx-size)*.1667);height:calc(var(--dfx-size)*.1667);border-radius:9999px;background:var(--dfx-color);transform-origin:calc(var(--dfx-size)*.0833) calc(var(--dfx-size)*.5);animation:dfx-dots-ring var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-dots-ring{0%,100%{transform:rotate(var(--dfx-rot)) scale(1);opacity:1}50%{transform:rotate(var(--dfx-rot)) scale(.5);opacity:.3}}
"#;

/// Eight dots on a ring that shrink and fade in a travelling wave.
#[component]
pub fn DotsRing(
    /// Width and height of the ring, in pixels.
    #[props(default = 48.0)]
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
        {dfx_style!("dots-ring", CSS)}
        div {
            class: "dfx dfx-loader dfx-dots-ring {class}",
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
