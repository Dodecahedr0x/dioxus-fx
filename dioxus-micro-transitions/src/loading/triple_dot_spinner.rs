use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-triple-dot-spinner{position:relative;width:var(--amt-size);height:var(--amt-size);animation:amt-triple-dot-spinner var(--amt-duration) linear infinite}
.amt-triple-dot-spinner span{position:absolute;top:0;left:50%;margin-left:calc(var(--amt-size)*-.1);width:calc(var(--amt-size)*.2);height:calc(var(--amt-size)*.2);border-radius:9999px;background:var(--amt-color);transform-origin:calc(var(--amt-size)*.1) calc(var(--amt-size)*.5)}
@keyframes amt-triple-dot-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// Three dots evenly spaced on a ring, turning as one.
#[component]
pub fn TripleDotSpinner(
    /// Diameter of the ring, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("triple-dot-spinner", CSS)}
        div {
            class: "amt amt-loader amt-triple-dot-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "transform:rotate({i * 120}deg);" }
            }
        }
    }
}
