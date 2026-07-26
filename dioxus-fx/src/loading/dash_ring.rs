use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-dash-ring{width:var(--dfx-size);height:var(--dfx-size)}
.dfx-dash-ring circle{stroke:var(--dfx-color);fill:none;stroke-width:3;stroke-dasharray:8 8;transform-origin:center;animation:dfx-dash-ring var(--dfx-duration) linear infinite}
@keyframes dfx-dash-ring{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("dash-ring", CSS)}
        svg {
            class: "dfx dfx-loader dfx-dash-ring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { cx: "25", cy: "25", r: "20" }
        }
    }
}
