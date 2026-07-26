use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-morphing-infinity{position:relative;width:var(--amt-size);height:calc(var(--amt-size)*.5)}
.amt-morphing-infinity span{position:absolute;top:0;left:0;width:calc(var(--amt-size)*.5);height:calc(var(--amt-size)*.5);border:2px solid var(--amt-color);border-radius:9999px;animation:var(--amt-duration) ease-in-out infinite}
.amt-morphing-infinity span:nth-child(1){animation-name:amt-morphing-infinity-a}
.amt-morphing-infinity span:nth-child(2){animation-name:amt-morphing-infinity-b}
@keyframes amt-morphing-infinity-a{0%,100%{transform:translateX(0) scale(1)}50%{transform:translateX(calc(var(--amt-size)*.5)) scale(.5)}}
@keyframes amt-morphing-infinity-b{0%,100%{transform:translateX(calc(var(--amt-size)*.5)) scale(.5)}50%{transform:translateX(0) scale(1)}}
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
        {amt_style!("morphing-infinity", CSS)}
        div {
            class: "amt amt-loader amt-morphing-infinity {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
