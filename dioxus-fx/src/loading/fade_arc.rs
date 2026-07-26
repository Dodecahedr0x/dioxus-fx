use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-fade-arc{width:var(--dfx-size);height:var(--dfx-size);animation:dfx-fade-arc var(--dfx-duration) linear infinite}
.dfx-fade-arc .dfx-fade-arc__track{stroke:var(--dfx-track);fill:none;stroke-width:3.5}
.dfx-fade-arc .dfx-fade-arc__arc{stroke:var(--dfx-color);fill:none;stroke-width:3.5;stroke-dasharray:80;stroke-dashoffset:28;stroke-linecap:round}
@keyframes dfx-fade-arc{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A fixed arc riding a faint track, spinning at a constant rate.
#[component]
pub fn FadeArc(
    /// Width and height of the loader, in pixels.
    #[props(default = 36.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("fade-arc", CSS)}
        svg {
            class: "dfx dfx-loader dfx-fade-arc {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { class: "dfx-fade-arc__track", cx: "25", cy: "25", r: "20" }
            circle { class: "dfx-fade-arc__arc", cx: "25", cy: "25", r: "20" }
        }
    }
}
