use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-newtons-cradle{display:flex;align-items:center;justify-content:center;gap:2px}
.amt-newtons-cradle span{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);transform-origin:top center}
.amt-newtons-cradle span:first-child{animation:amt-newtons-cradle-l var(--amt-duration) ease-in-out infinite}
.amt-newtons-cradle span:last-child{animation:amt-newtons-cradle-r var(--amt-duration) ease-in-out infinite}
@keyframes amt-newtons-cradle-l{0%{transform:rotate(25deg)}20%,80%{transform:rotate(0)}100%{transform:rotate(25deg)}}
@keyframes amt-newtons-cradle-r{0%,40%{transform:rotate(0)}60%{transform:rotate(-25deg)}80%,100%{transform:rotate(0)}}
"#;

/// Four suspended balls; the outermost two swing and hand off momentum.
#[component]
pub fn NewtonsCradle(
    /// Diameter of one ball, in pixels.
    #[props(default = 12.0)]
    size: f64,
    /// Ball colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full swing cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("newtons-cradle", CSS)}
        div {
            class: "amt amt-loader amt-newtons-cradle {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}" }
            }
        }
    }
}
