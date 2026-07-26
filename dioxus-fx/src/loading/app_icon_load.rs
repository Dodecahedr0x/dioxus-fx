use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-app-icon{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border-radius:calc(var(--dfx-size)*.25);background:var(--dfx-track);overflow:hidden}
.dfx-app-icon svg{width:calc(var(--dfx-size)*.667);height:calc(var(--dfx-size)*.667);transform:rotate(-90deg)}
.dfx-app-icon .dfx-app-icon__track{stroke:var(--dfx-track);fill:none;stroke-width:4}
.dfx-app-icon .dfx-app-icon__arc{stroke:var(--dfx-color);fill:none;stroke-width:4;stroke-dasharray:125;animation:dfx-app-icon var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-app-icon{from{stroke-dashoffset:125}to{stroke-dashoffset:0}}
"#;

/// A rounded app tile with a progress ring sweeping inside it.
#[component]
pub fn AppIconLoad(
    /// Width and height of the tile, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Ring colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full sweep, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("app-icon-load", CSS)}
        div {
            class: "dfx dfx-loader dfx-app-icon {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            svg { view_box: "0 0 50 50",
                circle { class: "dfx-app-icon__track", cx: "25", cy: "25", r: "20" }
                circle { class: "dfx-app-icon__arc", cx: "25", cy: "25", r: "20" }
            }
        }
    }
}
