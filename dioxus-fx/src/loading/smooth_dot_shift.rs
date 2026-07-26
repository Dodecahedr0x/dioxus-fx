use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-smooth-dot-shift{position:relative;display:flex;align-items:center;width:var(--dfx-size);height:calc(var(--dfx-size)*.25)}
.dfx-smooth-dot-shift i{position:absolute;width:calc(var(--dfx-size)*.25);height:calc(var(--dfx-size)*.25);border-radius:9999px;background:var(--dfx-track)}
.dfx-smooth-dot-shift i:nth-of-type(1){left:0}
.dfx-smooth-dot-shift i:nth-of-type(2){left:37.5%}
.dfx-smooth-dot-shift i:nth-of-type(3){left:75%}
.dfx-smooth-dot-shift span{position:absolute;left:0;z-index:1;width:calc(var(--dfx-size)*.25);height:calc(var(--dfx-size)*.25);border-radius:9999px;background:var(--dfx-color);animation:dfx-smooth-dot-shift var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-smooth-dot-shift{0%,100%{transform:translateX(0)}25%{transform:translateX(calc(var(--dfx-size)*.375))}50%{transform:translateX(calc(var(--dfx-size)*.75))}75%{transform:translateX(calc(var(--dfx-size)*.375))}}
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
        {dfx_style!("smooth-dot-shift", CSS)}
        div {
            class: "dfx dfx-loader dfx-smooth-dot-shift {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
