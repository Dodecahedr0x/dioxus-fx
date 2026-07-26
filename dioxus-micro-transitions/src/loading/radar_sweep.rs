use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-radar-sweep{position:relative;overflow:hidden;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);border:1px solid var(--amt-track);border-radius:9999px}
.amt-radar-sweep i{position:absolute;border:1px solid var(--amt-track);border-radius:9999px}
.amt-radar-sweep i:nth-of-type(1){width:66.6%;height:66.6%}
.amt-radar-sweep i:nth-of-type(2){width:33.3%;height:33.3%}
.amt-radar-sweep b{position:absolute;width:calc(var(--amt-size)*.125);height:calc(var(--amt-size)*.125);border-radius:9999px;background:var(--amt-color);opacity:.8}
.amt-radar-sweep span{position:absolute;inset:0;background:conic-gradient(from 0deg,transparent 50%,var(--amt-track) 85%,var(--amt-color) 100%);animation:amt-radar-sweep var(--amt-duration) linear infinite}
@keyframes amt-radar-sweep{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A radar dish: concentric rings with a bright sweep hand circling the dial.
#[component]
pub fn RadarSweep(
    /// Diameter of the dial, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Colour of the sweep hand and centre dot.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one sweep, in seconds.
    #[props(default = 1.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("radar-sweep", CSS)}
        div {
            class: "amt amt-loader amt-radar-sweep {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            i {}
            b {}
            span {}
        }
    }
}
