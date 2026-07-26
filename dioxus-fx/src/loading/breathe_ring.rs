use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-breathe-ring{width:var(--dfx-size);height:var(--dfx-size);border:4px solid var(--dfx-color);border-radius:9999px;animation:dfx-breathe-ring var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-breathe-ring{0%,100%{transform:scale(.8);border-width:6px}50%{transform:scale(1.1);border-width:2px}}
"#;

/// A ring that swells as its stroke thins, then draws back in.
#[component]
pub fn BreatheRing(
    /// Diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Border colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one breath, in seconds.
    #[props(default = 3.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("breathe-ring", CSS)}
        div {
            class: "dfx dfx-loader dfx-breathe-ring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
