use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-cube-flip-spring{width:var(--dfx-size);height:var(--dfx-size);border-radius:calc(var(--dfx-size)*.2);background:var(--dfx-color);animation:dfx-cube-flip-spring var(--dfx-duration) cubic-bezier(.34,1.4,.64,1) infinite}
@keyframes dfx-cube-flip-spring{0%{transform:perspective(200px) rotateX(0) rotateY(0)}50%{transform:perspective(200px) rotateX(180deg) rotateY(0)}100%{transform:perspective(200px) rotateX(180deg) rotateY(180deg)}}
"#;

/// A rounded tile flipping over one axis and then the other, with a little bounce.
#[component]
pub fn CubeFlipSpring(
    /// Width and height of the tile, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Fill colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("cube-flip-spring", CSS)}
        div {
            class: "dfx dfx-loader dfx-cube-flip-spring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
