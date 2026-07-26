use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-morphing-ring{width:var(--amt-size);height:var(--amt-size);border:3px solid var(--amt-color);animation:amt-morphing-ring var(--amt-duration) ease-in-out infinite}
@keyframes amt-morphing-ring{0%{border-radius:10%;transform:rotate(0)}50%{border-radius:50%;transform:rotate(90deg)}100%{border-radius:10%;transform:rotate(180deg)}}
"#;

/// An outlined square rounding itself into a ring and back as it turns.
#[component]
pub fn MorphingRing(
    /// Width and height of the shape, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Border colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("morphing-ring", CSS)}
        div {
            class: "amt amt-loader amt-morphing-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
