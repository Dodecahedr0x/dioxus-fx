use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-pendulum{position:relative;display:flex;align-items:flex-start;justify-content:center;width:var(--dfx-size);height:calc(var(--dfx-size)*.75)}
.dfx-pendulum b{position:absolute;top:0;width:100%;height:calc(var(--dfx-size)*.0625);border-radius:9999px;background:var(--dfx-track)}
.dfx-pendulum div{position:absolute;top:0;display:flex;flex-direction:column;align-items:center;transform-origin:top center;animation:dfx-pendulum var(--dfx-duration) ease-in-out infinite}
.dfx-pendulum i{width:calc(var(--dfx-size)*.0625);height:calc(var(--dfx-size)*.5);background:var(--dfx-color)}
.dfx-pendulum span{width:calc(var(--dfx-size)*.25);height:calc(var(--dfx-size)*.25);border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-pendulum{0%,100%{transform:rotate(-45deg)}50%{transform:rotate(45deg)}}
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
        {dfx_style!("pendulum", CSS)}
        div {
            class: "dfx dfx-loader dfx-pendulum {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
