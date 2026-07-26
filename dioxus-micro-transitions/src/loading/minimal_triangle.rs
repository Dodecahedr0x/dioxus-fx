use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-minimal-triangle{overflow:visible;width:var(--amt-size);height:var(--amt-size)}
.amt-minimal-triangle .amt-minimal-triangle__track{stroke:var(--amt-track);fill:none;stroke-width:3}
.amt-minimal-triangle .amt-minimal-triangle__arc{stroke:var(--amt-color);fill:none;stroke-width:3;stroke-dasharray:120;animation:amt-minimal-triangle var(--amt-duration) ease-in-out infinite}
@keyframes amt-minimal-triangle{0%{stroke-dashoffset:120}50%{stroke-dashoffset:0}100%{stroke-dashoffset:-120}}
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
        {amt_style!("minimal-triangle", CSS)}
        svg {
            class: "amt amt-loader amt-minimal-triangle {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            polygon { class: "amt-minimal-triangle__track", points: POINTS }
            polygon { class: "amt-minimal-triangle__arc", points: POINTS }
        }
    }
}
