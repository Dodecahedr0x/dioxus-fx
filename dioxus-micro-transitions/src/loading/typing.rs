use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-typing{display:flex;align-items:center;font-weight:500;color:var(--amt-color)}
.amt-typing span{display:inline-block;margin-left:4px;width:6px;height:16px;background:var(--amt-color);animation:amt-typing var(--amt-duration) steps(1) infinite}
@keyframes amt-typing{0%,100%{opacity:1}50%{opacity:0}}
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
        {amt_style!("typing", CSS)}
        div {
            class: "amt amt-loader amt-typing {class}",
            style: "--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
            span { aria_hidden: "true" }
        }
    }
}
