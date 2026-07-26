use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-floating-diamonds{display:flex;align-items:center;gap:calc(var(--dfx-size)*.667)}
.dfx-floating-diamonds span{width:var(--dfx-size);height:var(--dfx-size);border-radius:3px;background:var(--dfx-color);animation:dfx-floating-diamonds var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-floating-diamonds{0%,100%{transform:rotate(45deg) translateY(0)}50%{transform:rotate(45deg) translateY(calc(var(--dfx-size)*-.833))}}
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
        {dfx_style!("floating-diamonds", CSS)}
        div {
            class: "dfx dfx-loader dfx-floating-diamonds {class}",
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
