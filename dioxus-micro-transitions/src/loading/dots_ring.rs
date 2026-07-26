use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-dots-ring{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-dots-ring span{position:absolute;top:0;left:calc(50% - var(--amt-size)*.0833);width:calc(var(--amt-size)*.1667);height:calc(var(--amt-size)*.1667);border-radius:9999px;background:var(--amt-color);transform-origin:calc(var(--amt-size)*.0833) calc(var(--amt-size)*.5);animation:amt-dots-ring var(--amt-duration) ease-in-out infinite}
@keyframes amt-dots-ring{0%,100%{transform:rotate(var(--amt-rot)) scale(1);opacity:1}50%{transform:rotate(var(--amt-rot)) scale(.5);opacity:.3}}
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
        {amt_style!("dots-ring", CSS)}
        div {
            class: "amt amt-loader amt-dots-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..8 {
                span {
                    key: "{i}",
                    style: "--amt-rot:{i * 45}deg;animation-delay:{i as f64 * 0.15}s;",
                }
            }
        }
    }
}
