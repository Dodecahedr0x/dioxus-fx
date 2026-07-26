use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-classic-spinner{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-classic-spinner span{position:absolute;top:0;left:calc(50% - var(--amt-size)*.0625);width:calc(var(--amt-size)*.125);height:calc(var(--amt-size)*.25);background:var(--amt-color);border-radius:9999px;transform-origin:calc(var(--amt-size)*.0625) calc(var(--amt-size)*.5);animation:amt-classic-spinner var(--amt-duration) linear infinite}
@keyframes amt-classic-spinner{from{opacity:1}to{opacity:.2}}
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
        {amt_style!("classic-spinner", CSS)}
        div {
            class: "amt amt-loader amt-classic-spinner {class}",
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
