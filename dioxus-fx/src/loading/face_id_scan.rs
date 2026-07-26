use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-face-id-scan{position:relative;overflow:hidden;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);border:2px solid var(--dfx-track);border-radius:calc(var(--dfx-size)*.25)}
.dfx-face-id-scan svg{width:50%;height:50%;fill:var(--dfx-track)}
.dfx-face-id-scan span{position:absolute;left:0;width:100%;height:calc(var(--dfx-size)*.667);background:linear-gradient(to bottom,transparent,var(--dfx-scan));border-bottom:1px solid var(--dfx-color);animation:dfx-face-id-scan var(--dfx-duration) linear infinite}
@keyframes dfx-face-id-scan{0%{top:-100%}50%{top:100%}100%{top:-100%}}
"#;

/// A framed face outline with a scan line sweeping down and back.
#[component]
pub fn FaceIdScan(
    /// Width and height of the frame, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Colour of the scan line.
    #[props(default = "#22c55e".to_string())]
    color: String,
    /// Colour of the gradient trailing the scan line.
    #[props(default = "rgba(74,222,128,.3)".to_string())]
    scan_color: String,
    /// Length of one full sweep, in seconds.
    #[props(default = 2.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("face-id-scan", CSS)}
        div {
            class: "dfx dfx-loader dfx-face-id-scan {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-scan:{scan_color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            svg { view_box: "0 0 24 24",
                path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z" }
            }
            span {}
        }
    }
}
