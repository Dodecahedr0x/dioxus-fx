use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-typing{display:flex;align-items:center;font-weight:500;color:var(--dfx-color)}
.dfx-typing span{display:inline-block;margin-left:4px;width:6px;height:16px;background:var(--dfx-color);animation:dfx-typing var(--dfx-duration) steps(1) infinite}
@keyframes dfx-typing{0%,100%{opacity:1}50%{opacity:0}}
"#;

/// A word followed by a blinking text caret.
#[component]
pub fn Typing(
    /// The text shown before the caret.
    #[props(default = "Loading".to_string())]
    text: String,
    /// Text and caret colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one blink, in seconds.
    #[props(default = 0.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("typing", CSS)}
        div {
            class: "dfx dfx-loader dfx-typing {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
            span { aria_hidden: "true" }
        }
    }
}
