use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-grid-dots{display:grid;grid-template-columns:repeat(3,1fr);gap:calc(var(--dfx-size)*.6)}
.dfx-grid-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:dfx-grid-dots var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-grid-dots{0%,100%{transform:scale(1);opacity:1}50%{transform:scale(.5);opacity:.3}}
"#;

/// A three-by-three field of dots pulsing on a diagonal wave.
#[component]
pub fn GridDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 10.0)]
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
        {dfx_style!("grid-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-grid-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..9 {
                span {
                    key: "{i}",
                    style: "animation-delay:{((i % 3) + (i / 3)) as f64 * 0.2}s;",
                }
            }
        }
    }
}
