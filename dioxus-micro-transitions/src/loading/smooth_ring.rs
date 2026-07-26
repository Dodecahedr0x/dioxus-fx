use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-smooth-ring{width:var(--amt-size);height:var(--amt-size);animation:amt-smooth-ring var(--amt-duration) linear infinite}
.amt-smooth-ring .amt-smooth-ring__track{stroke:var(--amt-track);fill:none;stroke-width:3}
.amt-smooth-ring .amt-smooth-ring__arc{stroke:var(--amt-color);fill:none;stroke-width:3;stroke-dasharray:38 80;stroke-linecap:round}
@keyframes amt-smooth-ring{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A rounded arc on a faint track, turning smoothly.
#[component]
pub fn SmoothRing(
    /// Diameter, in pixels.
    #[props(default = 36.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("smooth-ring", CSS)}
        svg {
            class: "amt amt-loader amt-smooth-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 32 32",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { class: "amt-smooth-ring__track", cx: "16", cy: "16", r: "14" }
            circle { class: "amt-smooth-ring__arc", cx: "16", cy: "16", r: "14" }
        }
    }
}
