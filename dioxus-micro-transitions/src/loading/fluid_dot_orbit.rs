use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-fluid-dot-orbit{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-fluid-dot-orbit b{width:calc(var(--amt-size)*.25);height:calc(var(--amt-size)*.25);border-radius:9999px;background:var(--amt-color)}
.amt-fluid-dot-orbit div{position:absolute;inset:0;animation:amt-fluid-dot-orbit var(--amt-duration) linear infinite}
.amt-fluid-dot-orbit i{position:absolute;top:calc(var(--amt-size)*.1);left:50%;margin-left:calc(var(--amt-size)*-.1);width:calc(var(--amt-size)*.2);height:calc(var(--amt-size)*.2);border-radius:9999px;background:var(--amt-color);opacity:.55}
@keyframes amt-fluid-dot-orbit{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A still centre dot with a smaller satellite circling it.
#[component]
pub fn FluidDotOrbit(
    /// Diameter of the orbit, in pixels.
    #[props(default = 40.0)]
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
        {amt_style!("fluid-dot-orbit", CSS)}
        div {
            class: "amt amt-loader amt-fluid-dot-orbit {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            div {
                i {}
            }
        }
    }
}
