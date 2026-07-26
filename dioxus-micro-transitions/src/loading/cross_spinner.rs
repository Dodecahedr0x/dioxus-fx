use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-cross-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);animation:amt-cross-spinner var(--amt-duration) ease-in-out infinite}
.amt-cross-spinner span{position:absolute;background:var(--amt-color);border-radius:9999px}
.amt-cross-spinner span:nth-child(1){width:100%;height:calc(var(--amt-size)*.1875)}
.amt-cross-spinner span:nth-child(2){height:100%;width:calc(var(--amt-size)*.1875)}
@keyframes amt-cross-spinner{0%{transform:rotate(0)}25%{transform:rotate(90deg)}50%{transform:rotate(180deg)}75%{transform:rotate(270deg)}100%{transform:rotate(360deg)}}
"#;

/// A plus sign that steps around a full turn in quarters.
#[component]
pub fn CrossSpinner(
    /// Width and height of the cross, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("cross-spinner", CSS)}
        div {
            class: "amt amt-loader amt-cross-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
