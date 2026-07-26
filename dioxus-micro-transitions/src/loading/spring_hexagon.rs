use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-spring-hexagon{width:var(--amt-size);height:var(--amt-size);animation:amt-spring-hexagon var(--amt-duration) cubic-bezier(.34,1.4,.64,1) infinite}
.amt-spring-hexagon polygon{fill:var(--amt-color)}
@keyframes amt-spring-hexagon{0%{transform:scale(1) rotate(0)}50%{transform:scale(1.15) rotate(60deg)}100%{transform:scale(1) rotate(60deg)}}
"#;

/// A solid hexagon that swells and snaps a sixth of a turn.
#[component]
pub fn SpringHexagon(
    /// Width and height of the hexagon, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Fill colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("spring-hexagon", CSS)}
        svg {
            class: "amt amt-loader amt-spring-hexagon {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            polygon { points: "25,5 45,15 45,35 25,45 5,35 5,15" }
        }
    }
}
