use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-comet-spinner{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:conic-gradient(from 0deg,transparent 0%,var(--dfx-track) 60%,var(--dfx-color) 100%);-webkit-mask:radial-gradient(farthest-side,transparent calc(100% - var(--dfx-thickness)),#000 calc(100% - var(--dfx-thickness)));mask:radial-gradient(farthest-side,transparent calc(100% - var(--dfx-thickness)),#000 calc(100% - var(--dfx-thickness)));animation:dfx-comet-spinner var(--dfx-duration) linear infinite}
@keyframes dfx-comet-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("comet-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-comet-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;--dfx-thickness:{thickness}px;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
