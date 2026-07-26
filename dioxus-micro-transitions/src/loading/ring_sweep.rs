use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-ring-sweep{width:var(--amt-size);height:var(--amt-size);border:3px solid var(--amt-track);border-top-color:var(--amt-color);border-radius:9999px;animation:amt-ring-sweep var(--amt-duration) linear infinite}
@keyframes amt-ring-sweep{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// The plain spinner: a ring with one lit segment going round.
#[component]
pub fn RingSweep(
    /// Diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Colour of the lit segment.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("ring-sweep", CSS)}
        div {
            class: "amt amt-loader amt-ring-sweep {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
