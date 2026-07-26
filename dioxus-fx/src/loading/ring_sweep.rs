use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-ring-sweep{width:var(--dfx-size);height:var(--dfx-size);border:3px solid var(--dfx-track);border-top-color:var(--dfx-color);border-radius:9999px;animation:dfx-ring-sweep var(--dfx-duration) linear infinite}
@keyframes dfx-ring-sweep{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("ring-sweep", CSS)}
        div {
            class: "dfx dfx-loader dfx-ring-sweep {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
