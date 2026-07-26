use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-mac-terminal{display:flex;align-items:center;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:.875rem;color:var(--amt-color)}
.amt-mac-terminal span{display:inline-block;margin-left:6px;width:8px;height:16px;background:var(--amt-color);animation:amt-mac-terminal var(--amt-duration) steps(1) infinite}
@keyframes amt-mac-terminal{0%,100%{opacity:1}50%{opacity:0}}
"#;

/// A shell prompt with a blinking block cursor.
#[component]
pub fn MacTerminal(
    /// The prompt text shown before the cursor.
    #[props(default = "~ %".to_string())]
    prompt: String,
    /// Text and cursor colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one blink, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("mac-terminal", CSS)}
        div {
            class: "amt amt-loader amt-mac-terminal {class}",
            style: "--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            "{prompt}"
            span { aria_hidden: "true" }
        }
    }
}
