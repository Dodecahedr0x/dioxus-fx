use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-morph-dot-ring{display:grid;grid-template-columns:repeat(2,1fr);width:var(--dfx-size);height:var(--dfx-size);animation:dfx-morph-dot-ring var(--dfx-duration) ease-in-out infinite}
.dfx-morph-dot-ring span{width:100%;height:100%;border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-morph-dot-ring{0%{gap:4px;transform:rotate(0)}50%{gap:0;transform:rotate(90deg)}100%{gap:4px;transform:rotate(180deg)}}
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
        {dfx_style!("morph-dot-ring", CSS)}
        div {
            class: "dfx dfx-loader dfx-morph-dot-ring {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..4 {
                span { key: "{i}" }
            }
        }
    }
}
