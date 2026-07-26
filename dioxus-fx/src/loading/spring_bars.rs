use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-spring-bars{display:flex;flex-direction:column;justify-content:center;gap:8px;width:var(--dfx-size)}
.dfx-spring-bars span{height:calc(var(--dfx-size)*.15);border-radius:9999px;background:var(--dfx-color);transform-origin:left center;animation:dfx-spring-bars var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-spring-bars{0%,100%{transform:scaleX(.2)}50%{transform:scaleX(1)}}
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
        {dfx_style!("spring-bars", CSS)}
        div {
            class: "dfx dfx-loader dfx-spring-bars {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
