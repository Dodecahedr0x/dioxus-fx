use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-comet-spinner{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:conic-gradient(from 0deg,transparent 0%,var(--amt-track) 60%,var(--amt-color) 100%);-webkit-mask:radial-gradient(farthest-side,transparent calc(100% - var(--amt-thickness)),#000 calc(100% - var(--amt-thickness)));mask:radial-gradient(farthest-side,transparent calc(100% - var(--amt-thickness)),#000 calc(100% - var(--amt-thickness)));animation:amt-comet-spinner var(--amt-duration) linear infinite}
@keyframes amt-comet-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A ring with a comet tail that fades from solid to nothing as it spins.
#[component]
pub fn CometSpinner(
    /// Outer diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Colour at the head of the tail.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Ring thickness, in pixels.
    #[props(default = 4.0)]
    thickness: f64,
    /// Time for one revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("comet-spinner", CSS)}
        div {
            class: "amt amt-loader amt-comet-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;--amt-thickness:{thickness}px;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
