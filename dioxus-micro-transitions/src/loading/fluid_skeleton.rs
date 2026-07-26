use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-fluid-skeleton{position:relative;overflow:hidden;width:var(--amt-width);height:var(--amt-height);border-radius:var(--amt-radius);background:var(--amt-track)}
.amt-fluid-skeleton span{position:absolute;inset:0;background:linear-gradient(90deg,transparent,var(--amt-color),transparent);animation:amt-fluid-skeleton var(--amt-duration) linear infinite}
@keyframes amt-fluid-skeleton{from{transform:translateX(-100%)}to{transform:translateX(200%)}}
"#;

/// A rounded placeholder block with a highlight sweeping across it.
#[component]
pub fn FluidSkeleton(
    /// Width of the block, as a CSS length.
    #[props(default = "96px".to_string())]
    width: String,
    /// Height of the block, as a CSS length.
    #[props(default = "40px".to_string())]
    height: String,
    /// Corner radius, as a CSS length.
    #[props(default = "12px".to_string())]
    radius: String,
    /// Colour of the moving highlight.
    #[props(default = "rgba(255,255,255,.6)".to_string())]
    color: String,
    /// Time for one sweep, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("fluid-skeleton", CSS)}
        div {
            class: "amt amt-loader amt-fluid-skeleton {class}",
            style: "--amt-width:{width};--amt-height:{height};--amt-radius:{radius};--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
