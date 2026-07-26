use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-concentric-pulse{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-concentric-pulse span{position:absolute;border:1px solid var(--amt-color);border-radius:9999px;animation:amt-concentric-pulse var(--amt-duration) ease-out infinite}
@keyframes amt-concentric-pulse{from{width:0;height:0;opacity:1}to{width:var(--amt-size);height:var(--amt-size);opacity:0}}
"#;

/// Three hairline rings expanding out of a single point, like sonar.
#[component]
pub fn ConcentricPulse(
    /// Diameter the rings expand to, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one ring to travel from centre to edge, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("concentric-pulse", CSS)}
        div {
            class: "amt amt-loader amt-concentric-pulse {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * duration / 3.0}s;" }
            }
        }
    }
}
