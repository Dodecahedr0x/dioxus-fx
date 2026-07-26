use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-fluid-dot-orbit{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-fluid-dot-orbit b{width:calc(var(--dfx-size)*.25);height:calc(var(--dfx-size)*.25);border-radius:9999px;background:var(--dfx-color)}
.dfx-fluid-dot-orbit div{position:absolute;inset:0;animation:dfx-fluid-dot-orbit var(--dfx-duration) linear infinite}
.dfx-fluid-dot-orbit i{position:absolute;top:calc(var(--dfx-size)*.1);left:50%;margin-left:calc(var(--dfx-size)*-.1);width:calc(var(--dfx-size)*.2);height:calc(var(--dfx-size)*.2);border-radius:9999px;background:var(--dfx-color);opacity:.55}
@keyframes dfx-fluid-dot-orbit{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("fluid-dot-orbit", CSS)}
        div {
            class: "dfx dfx-loader dfx-fluid-dot-orbit {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
