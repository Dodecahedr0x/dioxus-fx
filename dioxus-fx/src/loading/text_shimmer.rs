use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-text-shimmer{position:relative;font-weight:500;color:var(--dfx-track)}
.dfx-text-shimmer span{position:absolute;inset:0;color:var(--dfx-color);-webkit-mask-image:linear-gradient(90deg,transparent 0%,#000 50%,transparent 100%);mask-image:linear-gradient(90deg,transparent 0%,#000 50%,transparent 100%);-webkit-mask-size:200% 100%;mask-size:200% 100%;animation:dfx-text-shimmer var(--dfx-duration) linear infinite}
@keyframes dfx-text-shimmer{from{-webkit-mask-position:100% 0;mask-position:100% 0}to{-webkit-mask-position:-100% 0;mask-position:-100% 0}}
"#;

/// Dimmed text with a bright band travelling across it.
#[component]
pub fn TextShimmer(
    /// The text to shimmer.
    #[props(default = "Thinking".to_string())]
    text: String,
    /// Colour of the highlighted portion.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one sweep, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("text-shimmer", CSS)}
        div {
            class: "dfx dfx-loader dfx-text-shimmer {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
            span { aria_hidden: "true", "{text}" }
        }
    }
}
