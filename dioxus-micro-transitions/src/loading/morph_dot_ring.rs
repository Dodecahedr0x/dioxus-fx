use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-morph-dot-ring{display:grid;grid-template-columns:repeat(2,1fr);width:var(--amt-size);height:var(--amt-size);animation:amt-morph-dot-ring var(--amt-duration) ease-in-out infinite}
.amt-morph-dot-ring span{width:100%;height:100%;border-radius:9999px;background:var(--amt-color)}
@keyframes amt-morph-dot-ring{0%{gap:4px;transform:rotate(0)}50%{gap:0;transform:rotate(90deg)}100%{gap:4px;transform:rotate(180deg)}}
"#;

/// Four dots that draw together into a single blob and rotate apart again.
#[component]
pub fn MorphDotRing(
    /// Width and height of the cluster, in pixels.
    #[props(default = 24.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("morph-dot-ring", CSS)}
        div {
            class: "amt amt-loader amt-morph-dot-ring {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}" }
            }
        }
    }
}
