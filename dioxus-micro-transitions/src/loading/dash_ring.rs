use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-dash-ring{width:var(--amt-size);height:var(--amt-size)}
.amt-dash-ring circle{stroke:var(--amt-color);fill:none;stroke-width:3;stroke-dasharray:8 8;transform-origin:center;animation:amt-dash-ring var(--amt-duration) linear infinite}
@keyframes amt-dash-ring{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A dashed circle rotating slowly, like a selection marquee.
#[component]
pub fn DashRing(
    /// Width and height of the ring, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Stroke colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 4.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("dash-ring", CSS)}
        svg {
            class: "amt amt-loader amt-dash-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { cx: "25", cy: "25", r: "20" }
        }
    }
}
