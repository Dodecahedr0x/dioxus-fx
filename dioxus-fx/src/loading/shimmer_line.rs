use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-shimmer-line{position:relative;overflow:hidden;width:var(--dfx-size);height:calc(var(--dfx-size)*.042);border-radius:9999px;background:var(--dfx-track)}
.dfx-shimmer-line span{position:absolute;top:0;bottom:0;left:0;width:33.3%;border-radius:9999px;background:var(--dfx-color);animation:dfx-shimmer-line var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-shimmer-line{from{transform:translateX(-100%)}to{transform:translateX(300%)}}
"#;

/// A slim indeterminate progress bar with a segment running along it.
#[component]
pub fn ShimmerLine(
    /// Width of the track, in pixels.
    #[props(default = 96.0)]
    size: f64,
    /// Colour of the moving segment.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one pass, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("shimmer-line", CSS)}
        div {
            class: "dfx dfx-loader dfx-shimmer-line {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
