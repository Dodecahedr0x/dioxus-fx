use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-swirling-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-swirling-spinner span{position:absolute;border:2.5px solid transparent;border-radius:9999px}
.amt-swirling-spinner span:nth-child(1){inset:0;border-top-color:var(--amt-color);border-right-color:color-mix(in srgb,var(--amt-color) 30%,transparent);border-bottom-color:color-mix(in srgb,var(--amt-color) 10%,transparent);animation:amt-spin-cw var(--amt-duration) linear infinite}
.amt-swirling-spinner span:nth-child(2){width:60%;height:60%;border-bottom-color:var(--amt-color);border-top-color:color-mix(in srgb,var(--amt-color) 30%,transparent);border-right-color:color-mix(in srgb,var(--amt-color) 10%,transparent);animation:amt-spin-ccw calc(var(--amt-duration)*.69) linear infinite}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes amt-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
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
        {amt_style!("swirling-spinner", CSS)}
        div {
            class: "amt amt-loader amt-swirling-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
