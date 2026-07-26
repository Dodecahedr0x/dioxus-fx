use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-morphing-shape{width:var(--amt-size);height:var(--amt-size);background:var(--amt-color);animation:amt-morphing-shape var(--amt-duration) ease-in-out infinite}
@keyframes amt-morphing-shape{0%{border-radius:10%;transform:rotate(0) scale(1)}50%{border-radius:50%;transform:rotate(90deg) scale(.8)}100%{border-radius:10%;transform:rotate(180deg) scale(1)}}
"#;

/// A solid square rounding into a disc and back while it turns and pulses.
#[component]
pub fn MorphingShape(
    /// Width and height of the shape, in pixels.
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
        {amt_style!("morphing-shape", CSS)}
        div {
            class: "amt amt-loader amt-morphing-shape {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
