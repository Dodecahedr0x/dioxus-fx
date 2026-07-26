use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-swapping-dots{position:relative;display:flex;align-items:center;width:var(--amt-size);height:calc(var(--amt-size)*.333)}
.amt-swapping-dots span{position:absolute;width:calc(var(--amt-size)*.333);height:calc(var(--amt-size)*.333);border-radius:9999px;background:var(--amt-color);animation:var(--amt-duration) ease-in-out infinite}
.amt-swapping-dots span:nth-child(1){left:0;animation-name:amt-swapping-dots-a}
.amt-swapping-dots span:nth-child(2){right:0;opacity:.5;animation-name:amt-swapping-dots-b}
@keyframes amt-swapping-dots-a{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--amt-size)*.667))}}
@keyframes amt-swapping-dots-b{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--amt-size)*-.667))}}
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
        {amt_style!("swapping-dots", CSS)}
        div {
            class: "amt amt-loader amt-swapping-dots {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
