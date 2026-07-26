use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-icon-morph{width:var(--dfx-size);height:var(--dfx-size);background:var(--dfx-color);animation:dfx-apple-icon-morph var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-apple-icon-morph{0%{border-radius:20%;transform:rotate(0)}33%{border-radius:50%;transform:rotate(90deg)}66%{border-radius:50%;transform:rotate(180deg)}100%{border-radius:20%;transform:rotate(270deg)}}
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
        {dfx_style!("apple-icon-morph", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-icon-morph {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
