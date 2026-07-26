use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-ios-spinner{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-ios-spinner span{position:absolute;top:calc(var(--dfx-size)*.06);left:calc(50% - var(--dfx-size)*.031);width:calc(var(--dfx-size)*.0625);height:calc(var(--dfx-size)*.22);border-radius:9999px;background:var(--dfx-color);transform-origin:calc(var(--dfx-size)*.031) calc(var(--dfx-size)*.44);animation:dfx-ios-spinner var(--dfx-duration) linear infinite}
@keyframes dfx-ios-spinner{from{opacity:1}to{opacity:.2}}
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
        {dfx_style!("ios-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-ios-spinner {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
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
