use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-fluid-bars{display:flex;align-items:flex-end;gap:calc(var(--dfx-size)*.125)}
.dfx-fluid-bars span{width:calc(var(--dfx-size)*.1875);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);transform-origin:bottom center;animation:dfx-fluid-bars var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-fluid-bars{0%,100%{transform:rotate(-15deg) scaleY(.8)}50%{transform:rotate(15deg) scaleY(1)}}
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
        {dfx_style!("fluid-bars", CSS)}
        div {
            class: "dfx dfx-loader dfx-fluid-bars {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
