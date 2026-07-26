use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-app-icon{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);border-radius:calc(var(--amt-size)*.25);background:var(--amt-track);overflow:hidden}
.amt-app-icon svg{width:calc(var(--amt-size)*.667);height:calc(var(--amt-size)*.667);transform:rotate(-90deg)}
.amt-app-icon .amt-app-icon__track{stroke:var(--amt-track);fill:none;stroke-width:4}
.amt-app-icon .amt-app-icon__arc{stroke:var(--amt-color);fill:none;stroke-width:4;stroke-dasharray:125;animation:amt-app-icon var(--amt-duration) ease-in-out infinite}
@keyframes amt-app-icon{from{stroke-dashoffset:125}to{stroke-dashoffset:0}}
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
        {amt_style!("app-icon-load", CSS)}
        div {
            class: "amt amt-loader amt-app-icon {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            svg { view_box: "0 0 50 50",
                circle { class: "amt-app-icon__track", cx: "25", cy: "25", r: "20" }
                circle { class: "amt-app-icon__arc", cx: "25", cy: "25", r: "20" }
            }
        }
    }
}
