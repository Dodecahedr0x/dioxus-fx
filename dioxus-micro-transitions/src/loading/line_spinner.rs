use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-line-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);border:2px solid var(--amt-track);border-radius:9999px}
.amt-line-spinner span{width:80%;height:calc(var(--amt-size)*.1);border-radius:9999px;background:var(--amt-color);animation:amt-line-spinner var(--amt-duration) linear infinite}
@keyframes amt-line-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {amt_style!("line-spinner", CSS)}
        div {
            class: "amt amt-loader amt-line-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
