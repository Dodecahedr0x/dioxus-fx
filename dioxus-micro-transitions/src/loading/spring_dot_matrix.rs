use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-spring-dot-matrix{display:grid;grid-template-columns:repeat(3,1fr);gap:calc(var(--amt-size)*1.25)}
.amt-spring-dot-matrix span{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);animation:amt-spring-dot-matrix var(--amt-duration) ease-in-out infinite}
@keyframes amt-spring-dot-matrix{0%,100%{transform:scale(1)}50%{transform:scale(.4)}}
"#;

/// A three-by-three dot matrix collapsing on a diagonal wave.
#[component]
pub fn SpringDotMatrix(
    /// Diameter of one dot, in pixels.
    #[props(default = 8.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("spring-dot-matrix", CSS)}
        div {
            class: "amt amt-loader amt-spring-dot-matrix {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..9 {
                span {
                    key: "{i}",
                    style: "animation-delay:{((i / 3) + (i % 3)) as f64 * 0.15}s;",
                }
            }
        }
    }
}
