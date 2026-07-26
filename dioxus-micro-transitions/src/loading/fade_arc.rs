use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-fade-arc{width:var(--amt-size);height:var(--amt-size);animation:amt-fade-arc var(--amt-duration) linear infinite}
.amt-fade-arc .amt-fade-arc__track{stroke:var(--amt-track);fill:none;stroke-width:3.5}
.amt-fade-arc .amt-fade-arc__arc{stroke:var(--amt-color);fill:none;stroke-width:3.5;stroke-dasharray:80;stroke-dashoffset:28;stroke-linecap:round}
@keyframes amt-fade-arc{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {amt_style!("fade-arc", CSS)}
        svg {
            class: "amt amt-loader amt-fade-arc {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            view_box: "0 0 50 50",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            circle { class: "amt-fade-arc__track", cx: "25", cy: "25", r: "20" }
            circle { class: "amt-fade-arc__arc", cx: "25", cy: "25", r: "20" }
        }
    }
}
