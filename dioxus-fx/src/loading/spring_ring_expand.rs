use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-spring-ring-expand{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-spring-ring-expand span{position:absolute;width:83%;height:83%;border:2px solid var(--dfx-color);border-radius:9999px;animation:dfx-spring-ring-expand var(--dfx-duration) ease-out infinite}
@keyframes dfx-spring-ring-expand{from{transform:scale(.1);opacity:1}to{transform:scale(1.25);opacity:0}}
"#;

/// Two rings blooming outward from a point, half a beat apart.
#[component]
pub fn SpringRingExpand(
    /// Width and height of the loader, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one ring to expand and fade, in seconds.
    #[props(default = 1.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("spring-ring-expand", CSS)}
        div {
            class: "dfx dfx-loader dfx-spring-ring-expand {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span { style: "animation-delay:{duration / 2.0}s;" }
        }
    }
}
