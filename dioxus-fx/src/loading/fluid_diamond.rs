use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-fluid-diamond{width:var(--dfx-size);height:var(--dfx-size);border-radius:3px;background:var(--dfx-color);animation:dfx-fluid-diamond var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-fluid-diamond{0%,100%{transform:rotate(45deg) scale(1,1)}50%{transform:rotate(45deg) scale(1.5,.5)}}
"#;

/// A diamond flattening and rebounding, as if made of liquid.
#[component]
pub fn FluidDiamond(
    /// Width and height of the diamond, in pixels.
    #[props(default = 24.0)]
    size: f64,
    /// Fill colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("fluid-diamond", CSS)}
        div {
            class: "dfx dfx-loader dfx-fluid-diamond {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
