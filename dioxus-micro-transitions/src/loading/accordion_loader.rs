use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-accordion{display:flex;flex-direction:column;justify-content:center;gap:calc(var(--amt-size)*.15);width:var(--amt-size);height:var(--amt-size)}
.amt-accordion span{width:100%;height:calc(var(--amt-size)*.1);background:var(--amt-color);border-radius:9999px;transform-origin:left center;animation:amt-accordion var(--amt-duration) ease-in-out infinite}
@keyframes amt-accordion{0%,100%{transform:scaleX(1)}50%{transform:scaleX(.2)}}
"#;

/// Four stacked bars that collapse and expand from the left in sequence.
#[component]
pub fn AccordionLoader(
    /// Width and height of the loader, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("accordion-loader", CSS)}
        div {
            class: "amt amt-loader amt-accordion {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
            }
        }
    }
}
