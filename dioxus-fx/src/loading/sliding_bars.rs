use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-sliding-bars{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-sliding-bars span{position:absolute;left:0;width:100%;height:calc(var(--dfx-size)*.1875);border-radius:9999px;background:var(--dfx-color);animation:var(--dfx-duration) ease-in-out infinite}
.dfx-sliding-bars span:nth-child(1){top:0;animation-name:dfx-sliding-bars-down}
.dfx-sliding-bars span:nth-child(2){bottom:0;opacity:.5;animation-name:dfx-sliding-bars-up}
@keyframes dfx-sliding-bars-down{0%,100%{transform:translateY(0)}50%{transform:translateY(calc(var(--dfx-size)*.75))}}
@keyframes dfx-sliding-bars-up{0%,100%{transform:translateY(0)}50%{transform:translateY(calc(var(--dfx-size)*-.75))}}
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
        {dfx_style!("sliding-bars", CSS)}
        div {
            class: "dfx dfx-loader dfx-sliding-bars {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
