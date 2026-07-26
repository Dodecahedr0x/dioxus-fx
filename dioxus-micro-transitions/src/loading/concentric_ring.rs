use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-concentric-ring{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-concentric-ring span{position:absolute;border:2px solid var(--amt-color);border-radius:9999px;animation:amt-spin-cw linear infinite}
.amt-concentric-ring span:nth-child(1){width:100%;height:100%;border-top-color:transparent;animation-duration:var(--amt-duration)}
.amt-concentric-ring span:nth-child(2){width:66.6%;height:66.6%;opacity:.65;border-bottom-color:transparent;animation-name:amt-spin-ccw;animation-duration:calc(var(--amt-duration)*.75)}
.amt-concentric-ring span:nth-child(3){width:33.3%;height:33.3%;opacity:.35;border-left-color:transparent;animation-duration:calc(var(--amt-duration)*.5)}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes amt-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// Three nested rings turning at different speeds, the middle one reversed.
#[component]
pub fn ConcentricRing(
    /// Outer diameter, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the outer ring to make one revolution, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("concentric-ring", CSS)}
        div {
            class: "amt amt-loader amt-concentric-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
            span {}
        }
    }
}
