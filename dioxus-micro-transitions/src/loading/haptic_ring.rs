use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-haptic-ring{width:var(--amt-size);height:var(--amt-size);border:3px solid var(--amt-track);border-top-color:var(--amt-color);border-radius:9999px;animation:amt-haptic-ring var(--amt-duration) cubic-bezier(.34,1.56,.64,1) infinite}
@keyframes amt-haptic-ring{0%{transform:rotate(0)}25%{transform:rotate(90deg)}50%{transform:rotate(180deg)}75%{transform:rotate(270deg)}100%{transform:rotate(360deg)}}
"#;

/// A ring that snaps through quarter turns with a springy overshoot.
#[component]
pub fn HapticRing(
    /// Diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Colour of the leading arc.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one full revolution, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("haptic-ring", CSS)}
        div {
            class: "amt amt-loader amt-haptic-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
