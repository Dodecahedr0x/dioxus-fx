use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-typing-indicator{display:inline-flex;align-items:center;justify-content:center;gap:4px;padding:8px 16px;border-radius:9999px;background:var(--amt-bg)}
.amt-typing-indicator span{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);animation:amt-typing-indicator var(--amt-duration) ease-in-out infinite}
@keyframes amt-typing-indicator{0%,100%{transform:translateY(0)}50%{transform:translateY(-4px)}}
"#;

/// The chat bubble with three dots that says someone is typing.
#[component]
pub fn TypingIndicator(
    /// Diameter of one dot, in pixels.
    #[props(default = 6.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Bubble background. Any CSS colour.
    #[props(default = "rgba(128,128,128,.15)".to_string())]
    background: String,
    /// Length of one bounce, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("typing-indicator", CSS)}
        div {
            class: "amt amt-loader amt-typing-indicator {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-bg:{background};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Typing",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
