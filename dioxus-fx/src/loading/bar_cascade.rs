use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-bar-cascade{display:flex;align-items:center;gap:4px;height:var(--dfx-size)}
.dfx-bar-cascade span{width:calc(var(--dfx-size)*.1875);border-radius:9999px;background:var(--dfx-color);animation:dfx-bar-cascade var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-bar-cascade{0%,100%{height:calc(var(--dfx-size)*.25)}50%{height:calc(var(--dfx-size)*.75)}}
"#;

/// Five bars rising and falling in a cascading run.
#[component]
pub fn BarCascade(
    /// Height of the frame, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("bar-cascade", CSS)}
        div {
            class: "dfx dfx-loader dfx-bar-cascade {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..5 {
                span { key: "{i}", style: "animation-delay:{i as f64 * 0.1}s;" }
            }
        }
    }
}
