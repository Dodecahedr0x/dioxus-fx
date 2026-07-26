use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-twin-orbit{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-twin-orbit span{position:absolute;border:2px solid var(--amt-color);border-radius:9999px}
.amt-twin-orbit span:nth-child(1){inset:0;border-bottom-color:transparent;border-right-color:transparent;animation:amt-spin-cw var(--amt-duration) linear infinite}
.amt-twin-orbit span:nth-child(2){inset:calc(var(--amt-size)*.167);opacity:.55;border-top-color:transparent;border-left-color:transparent;animation:amt-spin-ccw var(--amt-duration) linear infinite}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes amt-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// Two half-rings nested inside each other, turning opposite ways.
#[component]
pub fn TwinOrbit(
    /// Outer diameter, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("twin-orbit", CSS)}
        div {
            class: "amt amt-loader amt-twin-orbit {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
