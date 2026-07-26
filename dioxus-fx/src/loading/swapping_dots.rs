use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-swapping-dots{position:relative;display:flex;align-items:center;width:var(--dfx-size);height:calc(var(--dfx-size)*.333)}
.dfx-swapping-dots span{position:absolute;width:calc(var(--dfx-size)*.333);height:calc(var(--dfx-size)*.333);border-radius:9999px;background:var(--dfx-color);animation:var(--dfx-duration) ease-in-out infinite}
.dfx-swapping-dots span:nth-child(1){left:0;animation-name:dfx-swapping-dots-a}
.dfx-swapping-dots span:nth-child(2){right:0;opacity:.5;animation-name:dfx-swapping-dots-b}
@keyframes dfx-swapping-dots-a{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--dfx-size)*.667))}}
@keyframes dfx-swapping-dots-b{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--dfx-size)*-.667))}}
"#;

/// Two dots trading places and returning.
#[component]
pub fn SwappingDots(
    /// Distance between the dots' outer edges, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full swap, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("swapping-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-swapping-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
