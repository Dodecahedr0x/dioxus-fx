use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-pendulum{position:relative;display:flex;align-items:flex-start;justify-content:center;width:var(--amt-size);height:calc(var(--amt-size)*.75)}
.amt-pendulum b{position:absolute;top:0;width:100%;height:calc(var(--amt-size)*.0625);border-radius:9999px;background:var(--amt-track)}
.amt-pendulum div{position:absolute;top:0;display:flex;flex-direction:column;align-items:center;transform-origin:top center;animation:amt-pendulum var(--amt-duration) ease-in-out infinite}
.amt-pendulum i{width:calc(var(--amt-size)*.0625);height:calc(var(--amt-size)*.5);background:var(--amt-color)}
.amt-pendulum span{width:calc(var(--amt-size)*.25);height:calc(var(--amt-size)*.25);border-radius:9999px;background:var(--amt-color)}
@keyframes amt-pendulum{0%,100%{transform:rotate(-45deg)}50%{transform:rotate(45deg)}}
"#;

/// A weight on a rod swinging back and forth beneath a beam.
#[component]
pub fn Pendulum(
    /// Width of the beam, in pixels.
    #[props(default = 64.0)]
    size: f64,
    /// Rod and weight colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full swing, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("pendulum", CSS)}
        div {
            class: "amt amt-loader amt-pendulum {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            div {
                i {}
                span {}
            }
        }
    }
}
