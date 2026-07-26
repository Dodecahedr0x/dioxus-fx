use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-minimal-triangle{overflow:visible;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-minimal-triangle .dfx-minimal-triangle__track{stroke:var(--dfx-track);fill:none;stroke-width:3}
.dfx-minimal-triangle .dfx-minimal-triangle__arc{stroke:var(--dfx-color);fill:none;stroke-width:3;stroke-dasharray:120;animation:dfx-minimal-triangle var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-minimal-triangle{0%{stroke-dashoffset:120}50%{stroke-dashoffset:0}100%{stroke-dashoffset:-120}}
"#;

/// A triangle outline drawn on, then drawn off the far side.
#[component]
pub fn MinimalTriangle(
    /// Width and height of the triangle, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Stroke colour. Any CSS colour; defaults to the inherited text colour.
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
    const POINTS: &str = "25,5 45,40 5,40";
    rsx! {
        {dfx_style!("minimal-triangle", CSS)}
        svg {
            class: "dfx dfx-loader dfx-minimal-triangle {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            polygon { class: "dfx-minimal-triangle__track", points: POINTS }
            polygon { class: "dfx-minimal-triangle__arc", points: POINTS }
        }
    }
}
