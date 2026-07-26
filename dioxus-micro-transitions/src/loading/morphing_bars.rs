use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-morphing-bars{display:flex;align-items:center;height:var(--amt-size);animation:amt-morphing-bars-gap var(--amt-duration) ease-in-out infinite}
.amt-morphing-bars span{width:calc(var(--amt-size)*.25);border-radius:3px;background:var(--amt-color);animation:amt-morphing-bars var(--amt-duration) ease-in-out infinite}
@keyframes amt-morphing-bars-gap{0%,100%{gap:4px}50%{gap:0}}
@keyframes amt-morphing-bars{0%,100%{height:var(--amt-size)}50%{height:calc(var(--amt-size)*.5)}}
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
        {amt_style!("morphing-bars", CSS)}
        div {
            class: "amt amt-loader amt-morphing-bars {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}" }
            }
        }
    }
}
