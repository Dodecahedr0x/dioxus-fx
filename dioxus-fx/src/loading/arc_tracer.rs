use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-arc-tracer{width:var(--dfx-size);height:var(--dfx-size)}
.dfx-arc-tracer .dfx-arc-tracer__track{stroke:var(--dfx-track);fill:none;stroke-width:4}
.dfx-arc-tracer .dfx-arc-tracer__arc{stroke:var(--dfx-color);fill:none;stroke-width:4;stroke-linecap:round;stroke-dasharray:125;animation:dfx-arc-tracer var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-arc-tracer{0%{stroke-dashoffset:125}50%{stroke-dashoffset:0}100%{stroke-dashoffset:-125}}
"#;

/// An arc that draws itself around a track, then unwinds off the other end.
#[component]
pub fn ArcTracer(
    /// Width and height of the loader, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("arc-tracer", CSS)}
        svg {
            class: "dfx dfx-loader dfx-arc-tracer {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { class: "dfx-arc-tracer__track", cx: "25", cy: "25", r: "20" }
            circle { class: "dfx-arc-tracer__arc", cx: "25", cy: "25", r: "20" }
        }
    }
}
