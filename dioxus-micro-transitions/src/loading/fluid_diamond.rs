use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-fluid-diamond{width:var(--amt-size);height:var(--amt-size);border-radius:3px;background:var(--amt-color);animation:amt-fluid-diamond var(--amt-duration) ease-in-out infinite}
@keyframes amt-fluid-diamond{0%,100%{transform:rotate(45deg) scale(1,1)}50%{transform:rotate(45deg) scale(1.5,.5)}}
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
        {amt_style!("fluid-diamond", CSS)}
        div {
            class: "amt amt-loader amt-fluid-diamond {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
