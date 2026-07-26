use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-concentric-ring{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-concentric-ring span{position:absolute;border:2px solid var(--dfx-color);border-radius:9999px;animation:dfx-spin-cw linear infinite}
.dfx-concentric-ring span:nth-child(1){width:100%;height:100%;border-top-color:transparent;animation-duration:var(--dfx-duration)}
.dfx-concentric-ring span:nth-child(2){width:66.6%;height:66.6%;opacity:.65;border-bottom-color:transparent;animation-name:dfx-spin-ccw;animation-duration:calc(var(--dfx-duration)*.75)}
.dfx-concentric-ring span:nth-child(3){width:33.3%;height:33.3%;opacity:.35;border-left-color:transparent;animation-duration:calc(var(--dfx-duration)*.5)}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
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
        {dfx_style!("concentric-ring", CSS)}
        div {
            class: "dfx dfx-loader dfx-concentric-ring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
            span {}
        }
    }
}
