use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-double-ring{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-double-ring span{position:absolute;border:2px solid var(--amt-track);border-radius:9999px}
.amt-double-ring span:nth-child(1){width:100%;height:100%;border-top-color:var(--amt-color);animation:amt-spin-cw var(--amt-duration) linear infinite}
.amt-double-ring span:nth-child(2){width:60%;height:60%;border-bottom-color:var(--amt-color);animation:amt-spin-ccw calc(var(--amt-duration)*.667) linear infinite}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes amt-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// Two rings chasing each other in opposite directions.
#[component]
pub fn DoubleRing(
    /// Outer diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Colour of the leading arc on each ring.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the outer ring to make one revolution, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("double-ring", CSS)}
        div {
            class: "amt amt-loader amt-double-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
