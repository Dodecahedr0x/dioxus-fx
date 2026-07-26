use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-dual-arc{width:var(--amt-size);height:var(--amt-size);border:2px solid transparent;border-top-color:var(--amt-color);border-bottom-color:var(--amt-color);border-radius:9999px;animation:amt-dual-arc var(--amt-duration) linear infinite}
@keyframes amt-dual-arc{0%{transform:rotate(0) scale(1)}50%{transform:rotate(180deg) scale(.82)}100%{transform:rotate(360deg) scale(1)}}
"#;

/// Two opposing arcs spinning while the ring pumps in and out.
#[component]
pub fn DualArc(
    /// Diameter, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("dual-arc", CSS)}
        div {
            class: "amt amt-loader amt-dual-arc {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
