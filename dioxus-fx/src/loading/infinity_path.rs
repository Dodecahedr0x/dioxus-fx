use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-infinity-path{overflow:visible;width:var(--dfx-size);height:calc(var(--dfx-size)*.5)}
.dfx-infinity-path .dfx-infinity-path__track{stroke:var(--dfx-track);fill:none;stroke-width:4;stroke-linecap:round}
.dfx-infinity-path .dfx-infinity-path__arc{stroke:var(--dfx-color);fill:none;stroke-width:4;stroke-linecap:round;stroke-dasharray:100;animation:dfx-infinity-path var(--dfx-duration) linear infinite}
@keyframes dfx-infinity-path{from{stroke-dashoffset:100}to{stroke-dashoffset:-100}}
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
        {dfx_style!("infinity-path", CSS)}
        svg {
            class: "dfx dfx-loader dfx-infinity-path {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 60 30",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            path { class: "dfx-infinity-path__track", d: D }
            path { class: "dfx-infinity-path__arc", d: D }
        }
    }
}
