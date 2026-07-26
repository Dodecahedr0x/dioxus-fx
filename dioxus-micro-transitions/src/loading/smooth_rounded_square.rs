use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-smooth-rounded-square{width:var(--amt-size);height:var(--amt-size);border:3px solid var(--amt-color);animation:amt-smooth-rounded-square var(--amt-duration) ease-in-out infinite}
@keyframes amt-smooth-rounded-square{0%,100%{border-radius:10%}50%{border-radius:50%}}
"#;

/// An outlined square softening into a circle and firming back up.
#[component]
pub fn SmoothRoundedSquare(
    /// Width and height of the shape, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Border colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("smooth-rounded-square", CSS)}
        div {
            class: "amt amt-loader amt-smooth-rounded-square {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
