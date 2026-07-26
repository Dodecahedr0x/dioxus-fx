use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-ios-spinner{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-ios-spinner span{position:absolute;top:calc(var(--amt-size)*.06);left:calc(50% - var(--amt-size)*.031);width:calc(var(--amt-size)*.0625);height:calc(var(--amt-size)*.22);border-radius:9999px;background:var(--amt-color);transform-origin:calc(var(--amt-size)*.031) calc(var(--amt-size)*.44);animation:amt-ios-spinner var(--amt-duration) linear infinite}
@keyframes amt-ios-spinner{from{opacity:1}to{opacity:.2}}
"#;

/// The iOS activity indicator: twelve tapered spokes fading around the dial.
#[component]
pub fn IosSpinner(
    /// Width and height of the spinner, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Spoke colour. Any CSS colour; defaults to the inherited text colour.
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
        {amt_style!("ios-spinner", CSS)}
        div {
            class: "amt amt-loader amt-ios-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..12 {
                span {
                    key: "{i}",
                    style: "transform:rotate({i * 30}deg);animation-delay:{i as f64 * duration / 12.0}s;",
                }
            }
        }
    }
}
