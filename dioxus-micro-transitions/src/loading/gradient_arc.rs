use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-gradient-arc{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:linear-gradient(135deg,var(--amt-from) 0%,var(--amt-to) 100%);-webkit-mask-image:radial-gradient(transparent 55%,#000 60%);mask-image:radial-gradient(transparent 55%,#000 60%);animation:amt-gradient-arc var(--amt-duration) linear infinite}
@keyframes amt-gradient-arc{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A gradient-filled ring spinning at a constant rate.
#[component]
pub fn GradientArc(
    /// Diameter, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Colour at the start of the gradient.
    #[props(default = "#00f2fe".to_string())]
    from_color: String,
    /// Colour at the end of the gradient.
    #[props(default = "#4facfe".to_string())]
    to_color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("gradient-arc", CSS)}
        div {
            class: "amt amt-loader amt-gradient-arc {class}",
            style: "--amt-size:{size}px;--amt-from:{from_color};--amt-to:{to_color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
