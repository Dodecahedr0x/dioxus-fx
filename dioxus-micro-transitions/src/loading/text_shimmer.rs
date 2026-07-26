use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-text-shimmer{position:relative;font-weight:500;color:var(--amt-track)}
.amt-text-shimmer span{position:absolute;inset:0;color:var(--amt-color);-webkit-mask-image:linear-gradient(90deg,transparent 0%,#000 50%,transparent 100%);mask-image:linear-gradient(90deg,transparent 0%,#000 50%,transparent 100%);-webkit-mask-size:200% 100%;mask-size:200% 100%;animation:amt-text-shimmer var(--amt-duration) linear infinite}
@keyframes amt-text-shimmer{from{-webkit-mask-position:100% 0;mask-position:100% 0}to{-webkit-mask-position:-100% 0;mask-position:-100% 0}}
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
        {amt_style!("text-shimmer", CSS)}
        div {
            class: "amt amt-loader amt-text-shimmer {class}",
            style: "--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "{text}",
            ..attributes,
            "{text}"
            span { aria_hidden: "true", "{text}" }
        }
    }
}
