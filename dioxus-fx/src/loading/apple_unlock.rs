use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-unlock{position:relative;font-weight:500;font-size:.875rem;letter-spacing:.02em;user-select:none;color:var(--dfx-track)}
.dfx-apple-unlock span{position:absolute;inset:0;color:transparent;background-image:linear-gradient(90deg,transparent,var(--dfx-color),transparent);background-size:200% 100%;-webkit-background-clip:text;background-clip:text;animation:dfx-apple-unlock var(--dfx-duration) linear infinite}
@keyframes dfx-apple-unlock{from{background-position:200% 0}to{background-position:-200% 0}}
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
        {dfx_style!("apple-unlock", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-unlock {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
            span { aria_hidden: "true", "{text}" }
        }
    }
}
