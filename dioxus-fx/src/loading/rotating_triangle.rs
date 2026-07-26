use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-rotating-triangle{overflow:visible;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-rotating-triangle polygon{stroke:var(--dfx-color);fill:none;stroke-width:4;stroke-linejoin:round;transform-origin:center;animation:dfx-rotating-triangle var(--dfx-duration) linear infinite}
@keyframes dfx-rotating-triangle{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A triangle outline turning at a constant rate.
#[component]
pub fn RotatingTriangle(
    /// Width and height of the triangle, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Stroke colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("rotating-triangle", CSS)}
        svg {
            class: "dfx dfx-loader dfx-rotating-triangle {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            polygon { points: "25,5 45,40 5,40" }
        }
    }
}
