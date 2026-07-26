use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-text-reveal{position:relative;overflow:hidden;height:1.5em;font-weight:500;color:var(--dfx-color)}
.dfx-apple-text-reveal span{display:block;animation:dfx-apple-text-reveal var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-apple-text-reveal{0%{transform:translateY(100%)}50%{transform:translateY(0)}100%{transform:translateY(-100%)}}
"#;

/// A word that scrolls up through a fixed-height window, over and over.
#[component]
pub fn AppleTextReveal(
    /// The word to scroll.
    #[props(default = "Loading".to_string())]
    text: String,
    /// Text colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full pass, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("apple-text-reveal", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-text-reveal {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            span { aria_hidden: "true", "{text}" }
        }
    }
}
