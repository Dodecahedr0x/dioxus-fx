use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-trailing-dots{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-trailing-dots div{position:absolute;inset:0;animation:dfx-trailing-dots var(--dfx-duration) ease-in-out infinite}
.dfx-trailing-dots span{position:absolute;top:0;left:50%;margin-left:calc(var(--dfx-size)*-.1);width:calc(var(--dfx-size)*.2);height:calc(var(--dfx-size)*.2);border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-trailing-dots{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A lead dot orbiting with a fading tail of followers behind it.
#[component]
pub fn TrailingDots(
    /// Diameter of the orbit, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one orbit, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("trailing-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-trailing-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                div { key: "{i}", style: "animation-delay:-{i as f64 * 0.1}s;",
                    span { style: "opacity:{1.0 - i as f64 * 0.2};" }
                }
            }
        }
    }
}
