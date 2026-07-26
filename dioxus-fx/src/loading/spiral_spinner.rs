use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-spiral-spinner{width:var(--dfx-size);height:var(--dfx-size);border:3px solid var(--dfx-color);border-top-color:transparent;border-radius:9999px;animation:dfx-spiral-spinner var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-spiral-spinner{0%{transform:rotate(0) scale(1)}50%{transform:rotate(180deg) scale(.8)}100%{transform:rotate(360deg) scale(1)}}
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
        {dfx_style!("spiral-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-spiral-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
