use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-wandering-cube{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-wandering-cube span{position:absolute;top:0;left:0;width:40%;height:40%;border-radius:3px;background:var(--amt-color);animation:amt-wandering-cube var(--amt-duration) ease-in-out infinite}
@keyframes amt-wandering-cube{0%{transform:translate(0,0) rotate(0)}25%{transform:translate(calc(var(--amt-size)*.6),0) rotate(-90deg)}50%{transform:translate(calc(var(--amt-size)*.6),calc(var(--amt-size)*.6)) rotate(-180deg)}75%{transform:translate(0,calc(var(--amt-size)*.6)) rotate(-270deg)}100%{transform:translate(0,0) rotate(-360deg)}}
"#;

/// A square walking the perimeter of a box, tumbling as it goes.
#[component]
pub fn WanderingCube(
    /// Width and height of the box, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Square colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one full lap, in seconds.
    #[props(default = 2.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("wandering-cube", CSS)}
        div {
            class: "amt amt-loader amt-wandering-cube {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
