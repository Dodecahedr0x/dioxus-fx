use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-offset-rings{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-offset-rings span{position:absolute;border:2px solid var(--dfx-color);border-radius:9999px;animation:var(--dfx-duration) ease-in-out infinite}
.dfx-offset-rings span:nth-child(1){width:100%;height:100%;border-left-color:transparent;border-right-color:transparent;animation-name:dfx-spin-cw}
.dfx-offset-rings span:nth-child(2){width:66.6%;height:66.6%;opacity:.6;border-top-color:transparent;border-bottom-color:transparent;animation-name:dfx-spin-ccw;animation-delay:.2s}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// Two arc pairs at right angles, easing around in opposite directions.
#[component]
pub fn OffsetRings(
    /// Outer diameter, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("offset-rings", CSS)}
        div {
            class: "dfx dfx-loader dfx-offset-rings {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
