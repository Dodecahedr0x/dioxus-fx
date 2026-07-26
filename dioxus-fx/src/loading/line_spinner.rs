use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-line-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border:2px solid var(--dfx-track);border-radius:9999px}
.dfx-line-spinner span{width:80%;height:calc(var(--dfx-size)*.1);border-radius:9999px;background:var(--dfx-color);animation:dfx-line-spinner var(--dfx-duration) linear infinite}
@keyframes dfx-line-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A single needle sweeping inside a hairline circle.
#[component]
pub fn LineSpinner(
    /// Diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Needle colour. Any CSS colour; defaults to the inherited text colour.
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
        {dfx_style!("line-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-line-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
