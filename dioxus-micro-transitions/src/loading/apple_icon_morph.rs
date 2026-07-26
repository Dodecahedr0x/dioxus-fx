use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-apple-icon-morph{width:var(--amt-size);height:var(--amt-size);background:var(--amt-color);animation:amt-apple-icon-morph var(--amt-duration) ease-in-out infinite}
@keyframes amt-apple-icon-morph{0%{border-radius:20%;transform:rotate(0)}33%{border-radius:50%;transform:rotate(90deg)}66%{border-radius:50%;transform:rotate(180deg)}100%{border-radius:20%;transform:rotate(270deg)}}
"#;

/// A squircle that rounds itself into a circle and back while rotating.
#[component]
pub fn AppleIconMorph(
    /// Width and height of the shape, in pixels.
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
        {amt_style!("apple-icon-morph", CSS)}
        div {
            class: "amt amt-loader amt-apple-icon-morph {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
