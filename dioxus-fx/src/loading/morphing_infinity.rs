use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-morphing-infinity{position:relative;width:var(--dfx-size);height:calc(var(--dfx-size)*.5)}
.dfx-morphing-infinity span{position:absolute;top:0;left:0;width:calc(var(--dfx-size)*.5);height:calc(var(--dfx-size)*.5);border:2px solid var(--dfx-color);border-radius:9999px;animation:var(--dfx-duration) ease-in-out infinite}
.dfx-morphing-infinity span:nth-child(1){animation-name:dfx-morphing-infinity-a}
.dfx-morphing-infinity span:nth-child(2){animation-name:dfx-morphing-infinity-b}
@keyframes dfx-morphing-infinity-a{0%,100%{transform:translateX(0) scale(1)}50%{transform:translateX(calc(var(--dfx-size)*.5)) scale(.5)}}
@keyframes dfx-morphing-infinity-b{0%,100%{transform:translateX(calc(var(--dfx-size)*.5)) scale(.5)}50%{transform:translateX(0) scale(1)}}
"#;

/// Two rings trading places, each shrinking as the other grows.
#[component]
pub fn MorphingInfinity(
    /// Overall width, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full exchange, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("morphing-infinity", CSS)}
        div {
            class: "dfx dfx-loader dfx-morphing-infinity {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
