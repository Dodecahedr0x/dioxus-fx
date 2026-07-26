use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-expanding-cross{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-expanding-cross span{position:absolute;background:var(--dfx-color);border-radius:9999px}
.dfx-expanding-cross span:nth-child(1){top:50%;left:0;width:100%;height:calc(var(--dfx-size)*.125);margin-top:calc(var(--dfx-size)*-.0625);animation:dfx-expanding-cross-x var(--dfx-duration) ease-in-out infinite}
.dfx-expanding-cross span:nth-child(2){left:50%;top:0;height:100%;width:calc(var(--dfx-size)*.125);margin-left:calc(var(--dfx-size)*-.0625);animation:dfx-expanding-cross-y var(--dfx-duration) ease-in-out infinite;animation-delay:calc(var(--dfx-duration)*.5)}
@keyframes dfx-expanding-cross-x{0%,100%{transform:scaleX(.2)}50%{transform:scaleX(1)}}
@keyframes dfx-expanding-cross-y{0%,100%{transform:scaleY(.2)}50%{transform:scaleY(1)}}
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
        {dfx_style!("expanding-cross", CSS)}
        div {
            class: "dfx dfx-loader dfx-expanding-cross {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
