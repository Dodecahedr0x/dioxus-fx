use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-double-ring{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-double-ring span{position:absolute;border:2px solid var(--dfx-track);border-radius:9999px}
.dfx-double-ring span:nth-child(1){width:100%;height:100%;border-top-color:var(--dfx-color);animation:dfx-spin-cw var(--dfx-duration) linear infinite}
.dfx-double-ring span:nth-child(2){width:60%;height:60%;border-bottom-color:var(--dfx-color);animation:dfx-spin-ccw calc(var(--dfx-duration)*.667) linear infinite}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
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
        {dfx_style!("double-ring", CSS)}
        div {
            class: "dfx dfx-loader dfx-double-ring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
