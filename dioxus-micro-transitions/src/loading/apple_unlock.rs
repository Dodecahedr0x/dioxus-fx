use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-apple-unlock{position:relative;font-weight:500;font-size:.875rem;letter-spacing:.02em;user-select:none;color:var(--amt-track)}
.amt-apple-unlock span{position:absolute;inset:0;color:transparent;background-image:linear-gradient(90deg,transparent,var(--amt-color),transparent);background-size:200% 100%;-webkit-background-clip:text;background-clip:text;animation:amt-apple-unlock var(--amt-duration) linear infinite}
@keyframes amt-apple-unlock{from{background-position:200% 0}to{background-position:-200% 0}}
"#;

/// Dimmed text with a bright highlight sweeping across it, iOS lock-screen style.
#[component]
pub fn AppleUnlock(
    /// The phrase to shimmer.
    #[props(default = "Slide to unlock".to_string())]
    text: String,
    /// Colour of the moving highlight.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one sweep, in seconds.
    #[props(default = 2.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("apple-unlock", CSS)}
        div {
            class: "amt amt-loader amt-apple-unlock {class}",
            style: "--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
            span { aria_hidden: "true", "{text}" }
        }
    }
}
