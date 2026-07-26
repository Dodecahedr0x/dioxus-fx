use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-orbiting-dot{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-orbiting-dot b{width:calc(var(--amt-size)*.2);height:calc(var(--amt-size)*.2);border-radius:9999px;background:var(--amt-track)}
.amt-orbiting-dot div{position:absolute;inset:0;border:1px solid var(--amt-track);border-radius:9999px;animation:amt-spin-cw var(--amt-duration) linear infinite}
.amt-orbiting-dot i{position:absolute;top:0;left:50%;margin:calc(var(--amt-size)*-.15) 0 0 calc(var(--amt-size)*-.15);width:calc(var(--amt-size)*.3);height:calc(var(--amt-size)*.3);border-radius:9999px;background:var(--amt-color)}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A single bead running around a hairline track.
#[component]
pub fn OrbitingDot(
    /// Diameter of the track, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Bead colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one orbit, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("orbiting-dot", CSS)}
        div {
            class: "amt amt-loader amt-orbiting-dot {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            div {
                i {}
            }
        }
    }
}
