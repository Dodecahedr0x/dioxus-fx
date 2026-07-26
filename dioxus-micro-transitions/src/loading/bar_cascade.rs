use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-bar-cascade{display:flex;align-items:center;gap:4px;height:var(--amt-size)}
.amt-bar-cascade span{width:calc(var(--amt-size)*.1875);border-radius:9999px;background:var(--amt-color);animation:amt-bar-cascade var(--amt-duration) ease-in-out infinite}
@keyframes amt-bar-cascade{0%,100%{height:calc(var(--amt-size)*.25)}50%{height:calc(var(--amt-size)*.75)}}
"#;

/// Five bars rising and falling in a cascading run.
#[component]
pub fn BarCascade(
    /// Height of the frame, in pixels.
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
        {amt_style!("bar-cascade", CSS)}
        div {
            class: "amt amt-loader amt-bar-cascade {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
