use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-morph-loader{width:var(--dfx-size);height:var(--dfx-size);background:var(--dfx-color);box-shadow:0 10px 15px -3px rgba(0,0,0,.1);animation:dfx-morph-loader var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-morph-loader{0%{border-radius:25%;transform:rotate(0) scale(1)}25%{border-radius:50%;transform:rotate(90deg) scale(.8)}50%{border-radius:50%;transform:rotate(180deg) scale(1.1)}75%{border-radius:25%;transform:rotate(270deg) scale(.8)}100%{border-radius:25%;transform:rotate(360deg) scale(1)}}
"#;

/// A rounded square that morphs into a circle and back as it turns and pumps.
#[component]
pub fn MorphLoader(
    /// Width and height of the shape, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Fill colour. Any CSS colour.
    #[props(default = "#3b82f6".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 2.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("morph-loader", CSS)}
        div {
            class: "dfx dfx-loader dfx-morph-loader {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
