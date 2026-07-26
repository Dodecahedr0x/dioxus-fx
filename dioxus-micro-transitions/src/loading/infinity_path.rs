use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-infinity-path{overflow:visible;width:var(--amt-size);height:calc(var(--amt-size)*.5)}
.amt-infinity-path .amt-infinity-path__track{stroke:var(--amt-track);fill:none;stroke-width:4;stroke-linecap:round}
.amt-infinity-path .amt-infinity-path__arc{stroke:var(--amt-color);fill:none;stroke-width:4;stroke-linecap:round;stroke-dasharray:100;animation:amt-infinity-path var(--amt-duration) linear infinite}
@keyframes amt-infinity-path{from{stroke-dashoffset:100}to{stroke-dashoffset:-100}}
"#;

/// A stroke chasing itself around a lemniscate.
#[component]
pub fn InfinityPath(
    /// Width of the figure, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Stroke colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one full lap, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    const D: &str = "M 15 15 C 15 5, 25 5, 30 15 C 35 25, 45 25, 45 15 C 45 5, 35 5, 30 15 C 25 25, 15 25, 15 15";
    rsx! {
        {amt_style!("infinity-path", CSS)}
        svg {
            class: "amt amt-loader amt-infinity-path {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 60 30",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            path { class: "amt-infinity-path__track", d: D }
            path { class: "amt-infinity-path__arc", d: D }
        }
    }
}
