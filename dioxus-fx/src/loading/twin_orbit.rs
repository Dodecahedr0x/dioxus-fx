use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-twin-orbit{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-twin-orbit span{position:absolute;border:2px solid var(--dfx-color);border-radius:9999px}
.dfx-twin-orbit span:nth-child(1){inset:0;border-bottom-color:transparent;border-right-color:transparent;animation:dfx-spin-cw var(--dfx-duration) linear infinite}
.dfx-twin-orbit span:nth-child(2){inset:calc(var(--dfx-size)*.167);opacity:.55;border-top-color:transparent;border-left-color:transparent;animation:dfx-spin-ccw var(--dfx-duration) linear infinite}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
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
        {dfx_style!("twin-orbit", CSS)}
        div {
            class: "dfx dfx-loader dfx-twin-orbit {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
