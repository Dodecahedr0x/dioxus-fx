use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-ripple-effect{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-ripple-effect b{position:absolute;width:calc(var(--dfx-size)*.208);height:calc(var(--dfx-size)*.208);border-radius:9999px;background:var(--dfx-color)}
.dfx-ripple-effect span{position:absolute;width:66.6%;height:66.6%;border:1px solid var(--dfx-color);border-radius:9999px;animation:dfx-ripple-effect var(--dfx-duration) ease-out infinite}
@keyframes dfx-ripple-effect{from{transform:scale(.3);opacity:.8}to{transform:scale(1.6);opacity:0}}
"#;

/// A dot dropping rings outward like a stone in water.
#[component]
pub fn RippleEffect(
    /// Width and height of the loader, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Dot and ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one ring to expand and fade, in seconds.
    #[props(default = 2.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("ripple-effect", CSS)}
        div {
            class: "dfx dfx-loader dfx-ripple-effect {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            for i in 0..3 {
                span { key: "{i}", style: "animation-delay:{i as f64 * duration / 3.0}s;" }
            }
        }
    }
}
