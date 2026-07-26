use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-sliding-bars{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-sliding-bars span{position:absolute;left:0;width:100%;height:calc(var(--amt-size)*.1875);border-radius:9999px;background:var(--amt-color);animation:var(--amt-duration) ease-in-out infinite}
.amt-sliding-bars span:nth-child(1){top:0;animation-name:amt-sliding-bars-down}
.amt-sliding-bars span:nth-child(2){bottom:0;opacity:.5;animation-name:amt-sliding-bars-up}
@keyframes amt-sliding-bars-down{0%,100%{transform:translateY(0)}50%{transform:translateY(calc(var(--amt-size)*.75))}}
@keyframes amt-sliding-bars-up{0%,100%{transform:translateY(0)}50%{transform:translateY(calc(var(--amt-size)*-.75))}}
"#;

/// Two bars sliding past each other from opposite edges.
#[component]
pub fn SlidingBars(
    /// Width and height of the loader, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full pass, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("sliding-bars", CSS)}
        div {
            class: "amt amt-loader amt-sliding-bars {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
