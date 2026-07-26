use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-stacked-bar-pulse{display:flex;flex-direction:column;align-items:center;gap:4px;width:var(--dfx-size)}
.dfx-stacked-bar-pulse span{height:calc(var(--dfx-size)*.125);border-radius:9999px;background:var(--dfx-color);animation:dfx-stacked-bar-pulse var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-stacked-bar-pulse{0%,100%{width:50%;opacity:.2}50%{width:100%;opacity:1}}
"#;

/// Three centred bars widening and brightening in sequence.
#[component]
pub fn StackedBarPulse(
    /// Width of the widest bar, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("stacked-bar-pulse", CSS)}
        div {
            class: "dfx dfx-loader dfx-stacked-bar-pulse {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
