use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-expanding-cross{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-expanding-cross span{position:absolute;background:var(--amt-color);border-radius:9999px}
.amt-expanding-cross span:nth-child(1){top:50%;left:0;width:100%;height:calc(var(--amt-size)*.125);margin-top:calc(var(--amt-size)*-.0625);animation:amt-expanding-cross-x var(--amt-duration) ease-in-out infinite}
.amt-expanding-cross span:nth-child(2){left:50%;top:0;height:100%;width:calc(var(--amt-size)*.125);margin-left:calc(var(--amt-size)*-.0625);animation:amt-expanding-cross-y var(--amt-duration) ease-in-out infinite;animation-delay:calc(var(--amt-duration)*.5)}
@keyframes amt-expanding-cross-x{0%,100%{transform:scaleX(.2)}50%{transform:scaleX(1)}}
@keyframes amt-expanding-cross-y{0%,100%{transform:scaleY(.2)}50%{transform:scaleY(1)}}
"#;

/// A cross whose arms take turns stretching to full width and height.
#[component]
pub fn ExpandingCross(
    /// Width and height of the cross, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("expanding-cross", CSS)}
        div {
            class: "amt amt-loader amt-expanding-cross {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
