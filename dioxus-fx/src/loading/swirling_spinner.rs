use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-swirling-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-swirling-spinner span{position:absolute;border:2.5px solid transparent;border-radius:9999px}
.dfx-swirling-spinner span:nth-child(1){inset:0;border-top-color:var(--dfx-color);border-right-color:color-mix(in srgb,var(--dfx-color) 30%,transparent);border-bottom-color:color-mix(in srgb,var(--dfx-color) 10%,transparent);animation:dfx-spin-cw var(--dfx-duration) linear infinite}
.dfx-swirling-spinner span:nth-child(2){width:60%;height:60%;border-bottom-color:var(--dfx-color);border-top-color:color-mix(in srgb,var(--dfx-color) 30%,transparent);border-right-color:color-mix(in srgb,var(--dfx-color) 10%,transparent);animation:dfx-spin-ccw calc(var(--dfx-duration)*.69) linear infinite}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// Two tapered rings spinning against each other, each fading around its arc.
#[component]
pub fn SwirlingSpinner(
    /// Outer diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the outer ring to make one revolution, in seconds.
    #[props(default = 1.3)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("swirling-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-swirling-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
