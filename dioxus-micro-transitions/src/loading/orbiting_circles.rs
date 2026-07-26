use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-orbiting-circles{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-orbiting-circles b{width:calc(var(--amt-size)*.25);height:calc(var(--amt-size)*.25);border-radius:9999px;background:var(--amt-color)}
.amt-orbiting-circles div{position:absolute;inset:0;animation:var(--amt-duration) linear infinite}
.amt-orbiting-circles div:nth-of-type(1){animation-name:amt-spin-cw}
.amt-orbiting-circles div:nth-of-type(2){animation-name:amt-spin-ccw}
.amt-orbiting-circles i{position:absolute;top:0;left:50%;margin-left:calc(var(--amt-size)*-.0833);width:calc(var(--amt-size)*.1667);height:calc(var(--amt-size)*.1667);border-radius:9999px;background:var(--amt-color);opacity:.55}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes amt-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
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
        {amt_style!("orbiting-circles", CSS)}
        div {
            class: "amt amt-loader amt-orbiting-circles {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
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
