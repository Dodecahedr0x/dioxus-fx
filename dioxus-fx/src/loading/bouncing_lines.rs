use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bouncing-lines{display:flex;flex-direction:column;gap:6px;width:var(--dfx-size)}
.dfx-bouncing-lines span{width:100%;height:calc(var(--dfx-size)*.1875);border-radius:9999px;background:var(--dfx-color);transform-origin:left center;animation:dfx-bouncing-lines var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bouncing-lines{0%,100%{transform:scaleX(.3)}50%{transform:scaleX(1)}}
"#;

/// Three stacked lines extending from the left in sequence.
#[component]
pub fn BouncingLines(
    /// Width of the lines, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Line colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("bouncing-lines", CSS)}
        div {
            class: "dfx dfx-loader dfx-bouncing-lines {class}",
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
