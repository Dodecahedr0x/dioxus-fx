use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-mac-terminal{display:flex;align-items:center;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:.875rem;color:var(--dfx-color)}
.dfx-mac-terminal span{display:inline-block;margin-left:6px;width:8px;height:16px;background:var(--dfx-color);animation:dfx-mac-terminal var(--dfx-duration) steps(1) infinite}
@keyframes dfx-mac-terminal{0%,100%{opacity:1}50%{opacity:0}}
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
        {dfx_style!("mac-terminal", CSS)}
        div {
            class: "dfx dfx-loader dfx-mac-terminal {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            "{prompt}"
            span { aria_hidden: "true" }
        }
    }
}
