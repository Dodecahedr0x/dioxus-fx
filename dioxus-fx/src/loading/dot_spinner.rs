use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-dot-spinner{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-dot-spinner span{position:absolute;top:0;left:calc(50% - var(--dfx-size)*.1);width:calc(var(--dfx-size)*.2);height:calc(var(--dfx-size)*.2);border-radius:9999px;background:var(--dfx-color);transform-origin:calc(var(--dfx-size)*.1) calc(var(--dfx-size)*.5);animation:dfx-dot-spinner var(--dfx-duration) linear infinite}
@keyframes dfx-dot-spinner{from{opacity:1}to{opacity:.2}}
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
        {dfx_style!("dot-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-dot-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
