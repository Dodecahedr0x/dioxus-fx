use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-pulse-square{width:var(--dfx-size);height:var(--dfx-size);border:4px solid var(--dfx-color);animation:dfx-pulse-square var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-pulse-square{0%,100%{border-radius:20%;transform:scale(1);opacity:1}50%{border-radius:50%;transform:scale(1.2);opacity:.3}}
"#;

/// An outlined square swelling into a faded ring and snapping back.
#[component]
pub fn PulseSquare(
    /// Width and height of the shape, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Border colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("pulse-square", CSS)}
        div {
            class: "dfx dfx-loader dfx-pulse-square {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
