use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-hourglass{display:flex;flex-direction:column;align-items:center;justify-content:space-between;width:var(--dfx-size);height:calc(var(--dfx-size)*1.25);animation:dfx-hourglass var(--dfx-duration) ease-in-out infinite}
.dfx-hourglass span{width:0;height:0;border-left:calc(var(--dfx-size)*.5) solid transparent;border-right:calc(var(--dfx-size)*.5) solid transparent}
.dfx-hourglass span:nth-child(1){border-top:calc(var(--dfx-size)*.5) solid var(--dfx-color)}
.dfx-hourglass span:nth-child(2){border-bottom:calc(var(--dfx-size)*.5) solid var(--dfx-color)}
@keyframes dfx-hourglass{0%{transform:rotate(0)}40%,50%{transform:rotate(180deg)}90%,100%{transform:rotate(360deg)}}
"#;

/// Two stacked triangles flipping end over end, like sand running out.
#[component]
pub fn Hourglass(
    /// Width of the hourglass, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Fill colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 3.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("hourglass", CSS)}
        div {
            class: "dfx dfx-loader dfx-hourglass {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
