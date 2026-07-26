use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-smooth-ring{width:var(--dfx-size);height:var(--dfx-size);animation:dfx-smooth-ring var(--dfx-duration) linear infinite}
.dfx-smooth-ring .dfx-smooth-ring__track{stroke:var(--dfx-track);fill:none;stroke-width:3}
.dfx-smooth-ring .dfx-smooth-ring__arc{stroke:var(--dfx-color);fill:none;stroke-width:3;stroke-dasharray:38 80;stroke-linecap:round}
@keyframes dfx-smooth-ring{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("smooth-ring", CSS)}
        svg {
            class: "dfx dfx-loader dfx-smooth-ring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 32 32",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { class: "dfx-smooth-ring__track", cx: "16", cy: "16", r: "14" }
            circle { class: "dfx-smooth-ring__arc", cx: "16", cy: "16", r: "14" }
        }
    }
}
