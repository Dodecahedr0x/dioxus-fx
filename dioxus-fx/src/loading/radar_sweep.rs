use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-radar-sweep{position:relative;overflow:hidden;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border:1px solid var(--dfx-track);border-radius:9999px}
.dfx-radar-sweep i{position:absolute;border:1px solid var(--dfx-track);border-radius:9999px}
.dfx-radar-sweep i:nth-of-type(1){width:66.6%;height:66.6%}
.dfx-radar-sweep i:nth-of-type(2){width:33.3%;height:33.3%}
.dfx-radar-sweep b{position:absolute;width:calc(var(--dfx-size)*.125);height:calc(var(--dfx-size)*.125);border-radius:9999px;background:var(--dfx-color);opacity:.8}
.dfx-radar-sweep span{position:absolute;inset:0;background:conic-gradient(from 0deg,transparent 50%,var(--dfx-track) 85%,var(--dfx-color) 100%);animation:dfx-radar-sweep var(--dfx-duration) linear infinite}
@keyframes dfx-radar-sweep{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("radar-sweep", CSS)}
        div {
            class: "dfx dfx-loader dfx-radar-sweep {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
