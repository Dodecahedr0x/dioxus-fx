use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bar-sweep{position:relative;overflow:hidden;display:flex;align-items:center;width:var(--dfx-size);height:calc(var(--dfx-size)*.5);padding:4px;border:2px solid var(--dfx-color);border-radius:9999px}
.dfx-bar-sweep span{position:absolute;width:calc(var(--dfx-size)*.333);height:calc(var(--dfx-size)*.333);border-radius:9999px;background:var(--dfx-color);animation:dfx-bar-sweep var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bar-sweep{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--dfx-size)*.5))}}
"#;

/// A bead sliding from end to end inside a pill-shaped track.
#[component]
pub fn BarSweep(
    /// Width of the track, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Track and bead colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one round trip, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("bar-sweep", CSS)}
        div {
            class: "dfx dfx-loader dfx-bar-sweep {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
