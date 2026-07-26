use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-orbiting-circles{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-orbiting-circles b{width:calc(var(--dfx-size)*.25);height:calc(var(--dfx-size)*.25);border-radius:9999px;background:var(--dfx-color)}
.dfx-orbiting-circles div{position:absolute;inset:0;animation:var(--dfx-duration) linear infinite}
.dfx-orbiting-circles div:nth-of-type(1){animation-name:dfx-spin-cw}
.dfx-orbiting-circles div:nth-of-type(2){animation-name:dfx-spin-ccw}
.dfx-orbiting-circles i{position:absolute;top:0;left:50%;margin-left:calc(var(--dfx-size)*-.0833);width:calc(var(--dfx-size)*.1667);height:calc(var(--dfx-size)*.1667);border-radius:9999px;background:var(--dfx-color);opacity:.55}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// A fixed core with two satellites orbiting it in opposite directions.
#[component]
pub fn OrbitingCircles(
    /// Diameter of the orbit, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one orbit, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("orbiting-circles", CSS)}
        div {
            class: "dfx dfx-loader dfx-orbiting-circles {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            div {
                i {}
            }
            div {
                i {}
            }
        }
    }
}
