use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-triple-dot-spinner{position:relative;width:var(--dfx-size);height:var(--dfx-size);animation:dfx-triple-dot-spinner var(--dfx-duration) linear infinite}
.dfx-triple-dot-spinner span{position:absolute;top:0;left:50%;margin-left:calc(var(--dfx-size)*-.1);width:calc(var(--dfx-size)*.2);height:calc(var(--dfx-size)*.2);border-radius:9999px;background:var(--dfx-color);transform-origin:calc(var(--dfx-size)*.1) calc(var(--dfx-size)*.5)}
@keyframes dfx-triple-dot-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("triple-dot-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-triple-dot-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "transform:rotate({i * 120}deg);" }
            }
        }
    }
}
