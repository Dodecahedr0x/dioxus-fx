use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-floating-diamonds{display:flex;align-items:center;gap:calc(var(--amt-size)*.667)}
.amt-floating-diamonds span{width:var(--amt-size);height:var(--amt-size);border-radius:3px;background:var(--amt-color);animation:amt-floating-diamonds var(--amt-duration) ease-in-out infinite}
@keyframes amt-floating-diamonds{0%,100%{transform:rotate(45deg) translateY(0)}50%{transform:rotate(45deg) translateY(calc(var(--amt-size)*-.833))}}
"#;

/// Three diamonds bobbing up and down in a rolling wave.
#[component]
pub fn FloatingDiamonds(
    /// Width and height of one diamond, in pixels.
    #[props(default = 12.0)]
    size: f64,
    /// Diamond colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("floating-diamonds", CSS)}
        div {
            class: "amt amt-loader amt-floating-diamonds {class}",
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
