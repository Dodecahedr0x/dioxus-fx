use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-flip-square{width:var(--amt-size);height:var(--amt-size);border-radius:calc(var(--amt-size)*.19);background:var(--amt-color);animation:amt-flip-square var(--amt-duration) ease-in-out infinite}
@keyframes amt-flip-square{0%{transform:perspective(200px) rotateX(0) rotateY(0)}25%{transform:perspective(200px) rotateX(180deg) rotateY(0)}50%{transform:perspective(200px) rotateX(180deg) rotateY(180deg)}75%{transform:perspective(200px) rotateX(0) rotateY(180deg)}100%{transform:perspective(200px) rotateX(0) rotateY(0)}}
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
        {amt_style!("flip-square", CSS)}
        div {
            class: "amt amt-loader amt-flip-square {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
