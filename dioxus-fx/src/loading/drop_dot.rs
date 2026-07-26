use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-drop-dot{position:relative;display:flex;flex-direction:column;align-items:center;width:calc(var(--dfx-size)*.5);height:var(--dfx-size)}
.dfx-drop-dot b{position:absolute;top:0;z-index:1;width:calc(var(--dfx-size)*.208);height:calc(var(--dfx-size)*.208);border-radius:9999px;background:var(--dfx-color);animation:dfx-drop-dot var(--dfx-duration) cubic-bezier(.55,0,1,.45) infinite}
.dfx-drop-dot i{position:absolute;bottom:calc(var(--dfx-size)*.167);width:calc(var(--dfx-size)*.333);height:calc(var(--dfx-size)*.083);border-radius:9999px;background:var(--dfx-track);filter:blur(1px)}
@keyframes dfx-drop-dot{0%,100%{transform:translateY(0) scale(1,1)}50%{transform:translateY(calc(var(--dfx-size)*.5)) scale(.8,1.2)}}
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
        {dfx_style!("drop-dot", CSS)}
        div {
            class: "dfx dfx-loader dfx-drop-dot {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            i {}
        }
    }
}
