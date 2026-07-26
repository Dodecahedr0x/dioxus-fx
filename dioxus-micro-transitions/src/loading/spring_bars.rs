use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-spring-bars{display:flex;flex-direction:column;justify-content:center;gap:8px;width:var(--amt-size)}
.amt-spring-bars span{height:calc(var(--amt-size)*.15);border-radius:9999px;background:var(--amt-color);transform-origin:left center;animation:amt-spring-bars var(--amt-duration) ease-in-out infinite}
@keyframes amt-spring-bars{0%,100%{transform:scaleX(.2)}50%{transform:scaleX(1)}}
"#;

/// Three stacked bars stretching out from the left in sequence.
#[component]
pub fn SpringBars(
    /// Width of the bars, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.4)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("spring-bars", CSS)}
        div {
            class: "amt amt-loader amt-spring-bars {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
