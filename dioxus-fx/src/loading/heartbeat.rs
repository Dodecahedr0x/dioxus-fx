use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-heartbeat{overflow:visible;width:var(--dfx-size);height:calc(var(--dfx-size)*.5)}
.dfx-heartbeat polyline{stroke:var(--dfx-color);fill:none;stroke-width:2;stroke-linejoin:round;stroke-linecap:round;stroke-dasharray:100;animation:dfx-heartbeat var(--dfx-duration) linear infinite}
@keyframes dfx-heartbeat{from{stroke-dashoffset:100}to{stroke-dashoffset:-100}}
"#;

/// An ECG trace scrolling across the screen.
#[component]
pub fn Heartbeat(
    /// Width of the trace, in pixels.
    #[props(default = 64.0)]
    size: f64,
    /// Line colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the trace to travel one full length, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("heartbeat", CSS)}
        svg {
            class: "dfx dfx-loader dfx-heartbeat {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 64 32",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            polyline { points: "0,16 16,16 24,4 32,28 40,16 64,16" }
        }
    }
}
