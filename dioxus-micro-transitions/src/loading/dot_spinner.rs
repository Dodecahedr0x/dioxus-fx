use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-dot-spinner{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-dot-spinner span{position:absolute;top:0;left:calc(50% - var(--amt-size)*.1);width:calc(var(--amt-size)*.2);height:calc(var(--amt-size)*.2);border-radius:9999px;background:var(--amt-color);transform-origin:calc(var(--amt-size)*.1) calc(var(--amt-size)*.5);animation:amt-dot-spinner var(--amt-duration) linear infinite}
@keyframes amt-dot-spinner{from{opacity:1}to{opacity:.2}}
"#;

/// Eight dots on a ring, each dimming a beat after the last.
#[component]
pub fn DotSpinner(
    /// Width and height of the ring, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("dot-spinner", CSS)}
        div {
            class: "amt amt-loader amt-dot-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..8 {
                span {
                    key: "{i}",
                    style: "transform:rotate({i * 45}deg);animation-delay:{i as f64 * duration / 8.0}s;",
                }
            }
        }
    }
}
