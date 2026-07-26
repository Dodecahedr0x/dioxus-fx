use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-clock-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border:2px solid var(--dfx-color);border-radius:9999px}
.dfx-clock-spinner span{position:absolute;bottom:50%;width:calc(var(--dfx-size)*.05);background:var(--dfx-color);border-radius:9999px;transform-origin:bottom center;animation:dfx-clock-spinner linear infinite}
.dfx-clock-spinner .dfx-clock-spinner__second{height:calc(var(--dfx-size)*.35);animation-duration:var(--dfx-duration)}
.dfx-clock-spinner .dfx-clock-spinner__minute{height:calc(var(--dfx-size)*.25);animation-duration:calc(var(--dfx-duration)*6)}
@keyframes dfx-clock-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A clock face whose two hands sweep at a 6:1 ratio.
#[component]
pub fn ClockSpinner(
    /// Width and height of the clock, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Face and hand colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the fast hand to make one revolution, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("clock-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-clock-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span { class: "dfx-clock-spinner__second" }
            span { class: "dfx-clock-spinner__minute" }
        }
    }
}
