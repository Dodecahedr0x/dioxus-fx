use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-spring-text-pop{display:flex;font-weight:500;font-size:.875rem;letter-spacing:.1em;color:var(--dfx-color)}
.dfx-spring-text-pop span{display:inline-block;white-space:pre;transform-origin:bottom center;animation:dfx-spring-text-pop var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-spring-text-pop{0%,100%{transform:translateY(0)}50%{transform:translateY(-6px)}}
"#;

/// Text whose letters hop up one after another.
#[component]
pub fn SpringTextPop(
    /// The text to animate.
    #[props(default = "Loading...".to_string())]
    text: String,
    /// Text colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one hop, in seconds.
    #[props(default = 1.2)]
    duration: f64,
    /// Delay added per character, in seconds.
    #[props(default = 0.08)]
    stagger: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("spring-text-pop", CSS)}
        div {
            class: "dfx dfx-loader dfx-spring-text-pop {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            for (i , ch) in text.chars().enumerate() {
                span {
                    key: "{i}",
                    aria_hidden: "true",
                    style: "animation-delay:{i as f64 * stagger}s;",
                    "{ch}"
                }
            }
        }
    }
}
