use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-clock-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);border:2px solid var(--amt-color);border-radius:9999px}
.amt-clock-spinner span{position:absolute;bottom:50%;width:calc(var(--amt-size)*.05);background:var(--amt-color);border-radius:9999px;transform-origin:bottom center;animation:amt-clock-spinner linear infinite}
.amt-clock-spinner .amt-clock-spinner__second{height:calc(var(--amt-size)*.35);animation-duration:var(--amt-duration)}
.amt-clock-spinner .amt-clock-spinner__minute{height:calc(var(--amt-size)*.25);animation-duration:calc(var(--amt-duration)*6)}
@keyframes amt-clock-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {amt_style!("clock-spinner", CSS)}
        div {
            class: "amt amt-loader amt-clock-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span { class: "amt-clock-spinner__second" }
            span { class: "amt-clock-spinner__minute" }
        }
    }
}
