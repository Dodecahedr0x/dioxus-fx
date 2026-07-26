use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-smooth-dot-shift{position:relative;display:flex;align-items:center;width:var(--amt-size);height:calc(var(--amt-size)*.25)}
.amt-smooth-dot-shift i{position:absolute;width:calc(var(--amt-size)*.25);height:calc(var(--amt-size)*.25);border-radius:9999px;background:var(--amt-track)}
.amt-smooth-dot-shift i:nth-of-type(1){left:0}
.amt-smooth-dot-shift i:nth-of-type(2){left:37.5%}
.amt-smooth-dot-shift i:nth-of-type(3){left:75%}
.amt-smooth-dot-shift span{position:absolute;left:0;z-index:1;width:calc(var(--amt-size)*.25);height:calc(var(--amt-size)*.25);border-radius:9999px;background:var(--amt-color);animation:amt-smooth-dot-shift var(--amt-duration) ease-in-out infinite}
@keyframes amt-smooth-dot-shift{0%,100%{transform:translateX(0)}25%{transform:translateX(calc(var(--amt-size)*.375))}50%{transform:translateX(calc(var(--amt-size)*.75))}75%{transform:translateX(calc(var(--amt-size)*.375))}}
"#;

/// A filled dot hopping between three empty slots and back.
#[component]
pub fn SmoothDotShift(
    /// Overall width, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Colour of the moving dot.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full round trip, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("smooth-dot-shift", CSS)}
        div {
            class: "amt amt-loader amt-smooth-dot-shift {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            i {}
            i {}
            span {}
        }
    }
}
