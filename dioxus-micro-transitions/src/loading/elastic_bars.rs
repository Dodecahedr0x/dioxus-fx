use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-elastic-bars{display:flex;align-items:center;justify-content:center;gap:calc(var(--amt-size)*.2);height:var(--amt-size)}
.amt-elastic-bars span{width:calc(var(--amt-size)*.2);border-radius:9999px;background:var(--amt-color);animation:amt-elastic-bars var(--amt-duration) ease-in-out infinite}
@keyframes amt-elastic-bars{0%,100%{height:calc(var(--amt-size)*.2)}50%{height:calc(var(--amt-size)*.9)}}
"#;

/// Three bars stretching tall and snapping back, one after another.
#[component]
pub fn ElasticBars(
    /// Height of the tallest bar, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.1)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("elastic-bars", CSS)}
        div {
            class: "amt amt-loader amt-elastic-bars {class}",
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
