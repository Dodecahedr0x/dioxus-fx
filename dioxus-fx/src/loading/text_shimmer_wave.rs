use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-text-shimmer-wave{display:flex;font-weight:500;color:var(--dfx-color)}
.dfx-text-shimmer-wave span{display:inline-block;white-space:pre;animation:dfx-text-shimmer-wave var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-text-shimmer-wave{0%,100%{opacity:.3;transform:translateY(0)}50%{opacity:1;transform:translateY(-2px)}}
"#;

/// Text whose letters brighten and lift in a travelling wave.
#[component]
pub fn TextShimmerWave(
    /// The text to animate.
    #[props(default = "Thinking".to_string())]
    text: String,
    /// Text colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one wave, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Delay added per character, in seconds.
    #[props(default = 0.1)]
    stagger: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("text-shimmer-wave", CSS)}
        div {
            class: "dfx dfx-loader dfx-text-shimmer-wave {class}",
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
