use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-dashed-spiral{width:var(--amt-size);height:var(--amt-size);border:3px dashed var(--amt-color);border-radius:9999px;animation:amt-dashed-spiral var(--amt-duration) linear infinite}
@keyframes amt-dashed-spiral{0%{transform:rotate(0) scale(1)}50%{transform:rotate(180deg) scale(1.1)}100%{transform:rotate(360deg) scale(1)}}
"#;

/// A dashed ring turning while it breathes in and out.
#[component]
pub fn DashedSpiral(
    /// Width and height of the ring, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Border colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 4.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("dashed-spiral", CSS)}
        div {
            class: "amt amt-loader amt-dashed-spiral {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
