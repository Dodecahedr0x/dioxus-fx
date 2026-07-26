use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-text-blink{font-weight:500;color:var(--amt-color);animation:amt-text-blink var(--amt-duration) ease-in-out infinite}
@keyframes amt-text-blink{0%,100%{opacity:1}50%{opacity:.2}}
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
        {amt_style!("text-blink", CSS)}
        div {
            class: "amt amt-loader amt-text-blink {class}",
            style: "--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
        }
    }
}
