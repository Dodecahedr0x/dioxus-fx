use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-spiral-spinner{width:var(--amt-size);height:var(--amt-size);border:3px solid var(--amt-color);border-top-color:transparent;border-radius:9999px;animation:amt-spiral-spinner var(--amt-duration) ease-in-out infinite}
@keyframes amt-spiral-spinner{0%{transform:rotate(0) scale(1)}50%{transform:rotate(180deg) scale(.8)}100%{transform:rotate(360deg) scale(1)}}
"#;

/// An open ring that spins while drawing itself inward and out again.
#[component]
pub fn SpiralSpinner(
    /// Diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("spiral-spinner", CSS)}
        div {
            class: "amt amt-loader amt-spiral-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
