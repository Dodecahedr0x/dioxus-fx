use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-text-dots{display:flex;font-weight:500;color:var(--dfx-color)}
.dfx-text-dots em{display:flex;font-style:normal;padding-left:2px}
.dfx-text-dots em span{opacity:0;animation:dfx-text-dots var(--dfx-duration) linear infinite}
.dfx-text-dots em span:nth-child(1){animation-delay:0s}
.dfx-text-dots em span:nth-child(2){animation-delay:calc(var(--dfx-duration)*.2)}
.dfx-text-dots em span:nth-child(3){animation-delay:calc(var(--dfx-duration)*.4)}
@keyframes dfx-text-dots{0%{opacity:0}20%,80%{opacity:1}100%{opacity:0}}
"#;

/// A label followed by an ellipsis that types itself in and clears.
#[component]
pub fn TextDots(
    /// The label shown before the dots.
    #[props(default = "Thinking".to_string())]
    text: String,
    /// Text colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("text-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-text-dots {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            span { "{text}" }
            em { aria_hidden: "true",
                span { "." }
                span { "." }
                span { "." }
            }
        }
    }
}
