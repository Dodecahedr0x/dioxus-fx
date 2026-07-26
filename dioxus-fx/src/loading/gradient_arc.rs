use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-gradient-arc{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:linear-gradient(135deg,var(--dfx-from) 0%,var(--dfx-to) 100%);-webkit-mask-image:radial-gradient(transparent 55%,#000 60%);mask-image:radial-gradient(transparent 55%,#000 60%);animation:dfx-gradient-arc var(--dfx-duration) linear infinite}
@keyframes dfx-gradient-arc{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("gradient-arc", CSS)}
        div {
            class: "dfx dfx-loader dfx-gradient-arc {class}",
            style: "--dfx-size:{size}px;--dfx-from:{from_color};--dfx-to:{to_color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
