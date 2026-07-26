use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-orbiting-dot{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-orbiting-dot b{width:calc(var(--dfx-size)*.2);height:calc(var(--dfx-size)*.2);border-radius:9999px;background:var(--dfx-track)}
.dfx-orbiting-dot div{position:absolute;inset:0;border:1px solid var(--dfx-track);border-radius:9999px;animation:dfx-spin-cw var(--dfx-duration) linear infinite}
.dfx-orbiting-dot i{position:absolute;top:0;left:50%;margin:calc(var(--dfx-size)*-.15) 0 0 calc(var(--dfx-size)*-.15);width:calc(var(--dfx-size)*.3);height:calc(var(--dfx-size)*.3);border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("orbiting-dot", CSS)}
        div {
            class: "dfx dfx-loader dfx-orbiting-dot {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
