use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-hexagon-spinner{width:var(--dfx-size);height:var(--dfx-size)}
.dfx-hexagon-spinner .dfx-hexagon-spinner__track{stroke:var(--dfx-track);fill:none;stroke-width:4}
.dfx-hexagon-spinner .dfx-hexagon-spinner__arc{stroke:var(--dfx-color);fill:none;stroke-width:4;stroke-linecap:round;stroke-dasharray:120;animation:dfx-hexagon-spinner var(--dfx-duration) linear infinite}
@keyframes dfx-hexagon-spinner{from{stroke-dashoffset:120}to{stroke-dashoffset:-120}}
"#;

/// A hexagon outline being traced by a travelling stroke.
#[component]
pub fn HexagonSpinner(
    /// Width and height of the hexagon, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Stroke colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the stroke to travel one full loop, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    const POINTS: &str = "25,5 45,15 45,35 25,45 5,35 5,15";
    rsx! {
        {dfx_style!("hexagon-spinner", CSS)}
        svg {
            class: "dfx dfx-loader dfx-hexagon-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            polygon { class: "dfx-hexagon-spinner__track", points: POINTS }
            polygon { class: "dfx-hexagon-spinner__arc", points: POINTS }
        }
    }
}
