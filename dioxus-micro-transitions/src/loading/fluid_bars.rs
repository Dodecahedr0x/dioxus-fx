use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-fluid-bars{display:flex;align-items:flex-end;gap:calc(var(--amt-size)*.125)}
.amt-fluid-bars span{width:calc(var(--amt-size)*.1875);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);transform-origin:bottom center;animation:amt-fluid-bars var(--amt-duration) ease-in-out infinite}
@keyframes amt-fluid-bars{0%,100%{transform:rotate(-15deg) scaleY(.8)}50%{transform:rotate(15deg) scaleY(1)}}
"#;

/// Four bars swaying like reeds, each a beat behind the last.
#[component]
pub fn FluidBars(
    /// Bar height, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full sway, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("fluid-bars", CSS)}
        div {
            class: "amt amt-loader amt-fluid-bars {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
