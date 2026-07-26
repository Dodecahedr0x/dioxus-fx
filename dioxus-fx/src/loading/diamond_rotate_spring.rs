use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-diamond-rotate-spring{width:var(--dfx-size);height:var(--dfx-size);border-radius:3px;background:var(--dfx-color);animation:dfx-diamond-rotate-spring var(--dfx-duration) cubic-bezier(.34,1.56,.64,1) infinite}
@keyframes dfx-diamond-rotate-spring{0%{transform:rotate(45deg)}33%{transform:rotate(135deg)}66%{transform:rotate(225deg)}100%{transform:rotate(315deg)}}
"#;

/// A diamond that snaps a quarter-turn at a time, overshooting each stop.
#[component]
pub fn DiamondRotateSpring(
    /// Width and height of the diamond, in pixels.
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
        {dfx_style!("diamond-rotate-spring", CSS)}
        div {
            class: "dfx dfx-loader dfx-diamond-rotate-spring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
