use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-morphing-bars{display:flex;align-items:center;height:var(--dfx-size);animation:dfx-morphing-bars-gap var(--dfx-duration) ease-in-out infinite}
.dfx-morphing-bars span{width:calc(var(--dfx-size)*.25);border-radius:3px;background:var(--dfx-color);animation:dfx-morphing-bars var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-morphing-bars-gap{0%,100%{gap:4px}50%{gap:0}}
@keyframes dfx-morphing-bars{0%,100%{height:var(--dfx-size)}50%{height:calc(var(--dfx-size)*.5)}}
"#;

/// Three bars that shrink and slide together into a single block.
#[component]
pub fn MorphingBars(
    /// Height of the tallest bar, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("morphing-bars", CSS)}
        div {
            class: "dfx dfx-loader dfx-morphing-bars {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}" }
            }
        }
    }
}
