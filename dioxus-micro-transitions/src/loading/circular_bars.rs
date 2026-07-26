use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-circular-bars{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-circular-bars span{position:absolute;top:0;left:calc(50% - var(--amt-size)*.05);width:calc(var(--amt-size)*.1);height:calc(var(--amt-size)*.3);background:var(--amt-color);border-radius:9999px;transform-origin:calc(var(--amt-size)*.05) calc(var(--amt-size)*.5);animation:amt-circular-bars var(--amt-duration) ease-in-out infinite}
@keyframes amt-circular-bars{0%,100%{transform:rotate(var(--amt-rot)) scaleY(.5)}50%{transform:rotate(var(--amt-rot)) scaleY(1.5)}}
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
        {amt_style!("circular-bars", CSS)}
        div {
            class: "amt amt-loader amt-circular-bars {class}",
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
