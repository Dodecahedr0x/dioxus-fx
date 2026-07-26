use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-bouncing-bars{display:flex;align-items:center;gap:6px;height:var(--amt-size)}
.amt-bouncing-bars span{width:calc(var(--amt-size)*.1875);height:100%;border-radius:9999px;background:var(--amt-color);animation:amt-bouncing-bars var(--amt-duration) ease-in-out infinite}
@keyframes amt-bouncing-bars{0%,100%{transform:scaleY(.3)}50%{transform:scaleY(1)}}
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
        {amt_style!("bouncing-bars", CSS)}
        div {
            class: "amt amt-loader amt-bouncing-bars {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.2}s;" }
            }
        }
    }
}
