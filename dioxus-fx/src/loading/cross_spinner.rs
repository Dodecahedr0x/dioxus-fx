use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-cross-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);animation:dfx-cross-spinner var(--dfx-duration) ease-in-out infinite}
.dfx-cross-spinner span{position:absolute;background:var(--dfx-color);border-radius:9999px}
.dfx-cross-spinner span:nth-child(1){width:100%;height:calc(var(--dfx-size)*.1875)}
.dfx-cross-spinner span:nth-child(2){height:100%;width:calc(var(--dfx-size)*.1875)}
@keyframes dfx-cross-spinner{0%{transform:rotate(0)}25%{transform:rotate(90deg)}50%{transform:rotate(180deg)}75%{transform:rotate(270deg)}100%{transform:rotate(360deg)}}
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
        {dfx_style!("cross-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-cross-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
