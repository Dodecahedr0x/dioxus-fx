use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-arc-tracer{width:var(--amt-size);height:var(--amt-size)}
.amt-arc-tracer .amt-arc-tracer__track{stroke:var(--amt-track);fill:none;stroke-width:4}
.amt-arc-tracer .amt-arc-tracer__arc{stroke:var(--amt-color);fill:none;stroke-width:4;stroke-linecap:round;stroke-dasharray:125;animation:amt-arc-tracer var(--amt-duration) ease-in-out infinite}
@keyframes amt-arc-tracer{0%{stroke-dashoffset:125}50%{stroke-dashoffset:0}100%{stroke-dashoffset:-125}}
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
        {amt_style!("arc-tracer", CSS)}
        svg {
            class: "amt amt-loader amt-arc-tracer {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { class: "amt-arc-tracer__track", cx: "25", cy: "25", r: "20" }
            circle { class: "amt-arc-tracer__arc", cx: "25", cy: "25", r: "20" }
        }
    }
}
