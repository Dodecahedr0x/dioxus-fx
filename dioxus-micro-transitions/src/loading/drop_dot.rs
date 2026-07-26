use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-drop-dot{position:relative;display:flex;flex-direction:column;align-items:center;width:calc(var(--amt-size)*.5);height:var(--amt-size)}
.amt-drop-dot b{position:absolute;top:0;z-index:1;width:calc(var(--amt-size)*.208);height:calc(var(--amt-size)*.208);border-radius:9999px;background:var(--amt-color);animation:amt-drop-dot var(--amt-duration) cubic-bezier(.55,0,1,.45) infinite}
.amt-drop-dot i{position:absolute;bottom:calc(var(--amt-size)*.167);width:calc(var(--amt-size)*.333);height:calc(var(--amt-size)*.083);border-radius:9999px;background:var(--amt-track);filter:blur(1px)}
@keyframes amt-drop-dot{0%,100%{transform:translateY(0) scale(1,1)}50%{transform:translateY(calc(var(--amt-size)*.5)) scale(.8,1.2)}}
"#;

/// A droplet that falls onto a soft shadow, squashing as it lands.
#[component]
pub fn DropDot(
    /// Height of the fall, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Droplet colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one fall-and-return, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("drop-dot", CSS)}
        div {
            class: "amt amt-loader amt-drop-dot {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            i {}
        }
    }
}
