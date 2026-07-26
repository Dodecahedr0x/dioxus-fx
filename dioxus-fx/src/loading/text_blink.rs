use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-text-blink{font-weight:500;color:var(--dfx-color);animation:dfx-text-blink var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-text-blink{0%,100%{opacity:1}50%{opacity:.2}}
"#;

/// A word fading down and back up, over and over.
#[component]
pub fn TextBlink(
    /// The text to fade.
    #[props(default = "Thinking".to_string())]
    text: String,
    /// Text colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one fade cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("text-blink", CSS)}
        div {
            class: "dfx dfx-loader dfx-text-blink {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
        }
    }
}
