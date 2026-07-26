use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-breathe-ring{width:var(--amt-size);height:var(--amt-size);border:4px solid var(--amt-color);border-radius:9999px;animation:amt-breathe-ring var(--amt-duration) ease-in-out infinite}
@keyframes amt-breathe-ring{0%,100%{transform:scale(.8);border-width:6px}50%{transform:scale(1.1);border-width:2px}}
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
        {amt_style!("breathe-ring", CSS)}
        div {
            class: "amt amt-loader amt-breathe-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
