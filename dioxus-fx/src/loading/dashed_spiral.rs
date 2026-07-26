use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-dashed-spiral{width:var(--dfx-size);height:var(--dfx-size);border:3px dashed var(--dfx-color);border-radius:9999px;animation:dfx-dashed-spiral var(--dfx-duration) linear infinite}
@keyframes dfx-dashed-spiral{0%{transform:rotate(0) scale(1)}50%{transform:rotate(180deg) scale(1.1)}100%{transform:rotate(360deg) scale(1)}}
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
        {dfx_style!("dashed-spiral", CSS)}
        div {
            class: "dfx dfx-loader dfx-dashed-spiral {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
        }
    }
}
