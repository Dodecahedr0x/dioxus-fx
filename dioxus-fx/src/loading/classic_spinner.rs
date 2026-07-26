use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-classic-spinner{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-classic-spinner span{position:absolute;top:0;left:calc(50% - var(--dfx-size)*.0625);width:calc(var(--dfx-size)*.125);height:calc(var(--dfx-size)*.25);background:var(--dfx-color);border-radius:9999px;transform-origin:calc(var(--dfx-size)*.0625) calc(var(--dfx-size)*.5);animation:dfx-classic-spinner var(--dfx-duration) linear infinite}
@keyframes dfx-classic-spinner{from{opacity:1}to{opacity:.2}}
"#;

/// The twelve-spoke system spinner, each spoke fading in turn.
#[component]
pub fn ClassicSpinner(
    /// Width and height of the loader, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Spoke colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one full revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("classic-spinner", CSS)}
        div {
            class: "dfx dfx-loader dfx-classic-spinner {class}",
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
