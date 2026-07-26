use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bouncing-bars{display:flex;align-items:center;gap:6px;height:var(--dfx-size)}
.dfx-bouncing-bars span{width:calc(var(--dfx-size)*.1875);height:100%;border-radius:9999px;background:var(--dfx-color);animation:dfx-bouncing-bars var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bouncing-bars{0%,100%{transform:scaleY(.3)}50%{transform:scaleY(1)}}
"#;

/// Three bars squeezing and releasing along their length.
#[component]
pub fn BouncingBars(
    /// Height of the bars, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("bouncing-bars", CSS)}
        div {
            class: "dfx dfx-loader dfx-bouncing-bars {class}",
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
