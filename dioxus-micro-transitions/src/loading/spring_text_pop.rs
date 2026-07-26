use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-spring-text-pop{display:flex;font-weight:500;font-size:.875rem;letter-spacing:.1em;color:var(--amt-color)}
.amt-spring-text-pop span{display:inline-block;white-space:pre;transform-origin:bottom center;animation:amt-spring-text-pop var(--amt-duration) ease-in-out infinite}
@keyframes amt-spring-text-pop{0%,100%{transform:translateY(0)}50%{transform:translateY(-6px)}}
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
        {amt_style!("spring-text-pop", CSS)}
        div {
            class: "amt amt-loader amt-spring-text-pop {class}",
            style: "--amt-color:{color};--amt-duration:{duration}s;",
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
