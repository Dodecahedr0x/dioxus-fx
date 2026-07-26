use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-shimmer-line{position:relative;overflow:hidden;width:var(--amt-size);height:calc(var(--amt-size)*.042);border-radius:9999px;background:var(--amt-track)}
.amt-shimmer-line span{position:absolute;top:0;bottom:0;left:0;width:33.3%;border-radius:9999px;background:var(--amt-color);animation:amt-shimmer-line var(--amt-duration) ease-in-out infinite}
@keyframes amt-shimmer-line{from{transform:translateX(-100%)}to{transform:translateX(300%)}}
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
        {amt_style!("shimmer-line", CSS)}
        div {
            class: "amt amt-loader amt-shimmer-line {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
