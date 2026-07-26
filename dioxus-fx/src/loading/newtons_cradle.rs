use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-newtons-cradle{display:flex;align-items:center;justify-content:center;gap:2px}
.dfx-newtons-cradle span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);transform-origin:top center}
.dfx-newtons-cradle span:first-child{animation:dfx-newtons-cradle-l var(--dfx-duration) ease-in-out infinite}
.dfx-newtons-cradle span:last-child{animation:dfx-newtons-cradle-r var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-newtons-cradle-l{0%{transform:rotate(25deg)}20%,80%{transform:rotate(0)}100%{transform:rotate(25deg)}}
@keyframes dfx-newtons-cradle-r{0%,40%{transform:rotate(0)}60%{transform:rotate(-25deg)}80%,100%{transform:rotate(0)}}
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
        {dfx_style!("newtons-cradle", CSS)}
        div {
            class: "dfx dfx-loader dfx-newtons-cradle {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}" }
            }
        }
    }
}
