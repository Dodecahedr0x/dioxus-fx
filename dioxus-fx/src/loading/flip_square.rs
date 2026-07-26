use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-flip-square{width:var(--dfx-size);height:var(--dfx-size);border-radius:calc(var(--dfx-size)*.19);background:var(--dfx-color);animation:dfx-flip-square var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-flip-square{0%{transform:perspective(200px) rotateX(0) rotateY(0)}25%{transform:perspective(200px) rotateX(180deg) rotateY(0)}50%{transform:perspective(200px) rotateX(180deg) rotateY(180deg)}75%{transform:perspective(200px) rotateX(0) rotateY(180deg)}100%{transform:perspective(200px) rotateX(0) rotateY(0)}}
"#;

/// A tile flipping through all four faces of a two-axis cycle.
#[component]
pub fn FlipSquare(
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
        {dfx_style!("flip-square", CSS)}
        div {
            class: "dfx dfx-loader dfx-flip-square {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
