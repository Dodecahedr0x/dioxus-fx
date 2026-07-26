use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-breathing-square{width:var(--dfx-size);height:var(--dfx-size);background:var(--dfx-color);animation:dfx-breathing-square var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-breathing-square{0%{border-radius:0;transform:scale(1) rotate(0)}50%{border-radius:50%;transform:scale(1.2) rotate(90deg)}100%{border-radius:0;transform:scale(1) rotate(180deg)}}
"#;

/// A solid square inflating into a disc and back as it turns.
#[component]
pub fn BreathingSquare(
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
        {dfx_style!("breathing-square", CSS)}
        div {
            class: "dfx dfx-loader dfx-breathing-square {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
