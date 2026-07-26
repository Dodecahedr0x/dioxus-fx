use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-bar-sweep{position:relative;overflow:hidden;display:flex;align-items:center;width:var(--amt-size);height:calc(var(--amt-size)*.5);padding:4px;border:2px solid var(--amt-color);border-radius:9999px}
.amt-bar-sweep span{position:absolute;width:calc(var(--amt-size)*.333);height:calc(var(--amt-size)*.333);border-radius:9999px;background:var(--amt-color);animation:amt-bar-sweep var(--amt-duration) ease-in-out infinite}
@keyframes amt-bar-sweep{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--amt-size)*.5))}}
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
        {amt_style!("bar-sweep", CSS)}
        div {
            class: "amt amt-loader amt-bar-sweep {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
