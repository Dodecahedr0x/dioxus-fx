use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-dynamic-island{display:flex;align-items:center;justify-content:center;gap:12px;height:32px;padding:0 16px;border-radius:9999px;background:var(--dfx-color);color:#fff;animation:dfx-dynamic-island var(--dfx-duration) ease-in-out infinite}
.dfx-dynamic-island b{width:6px;height:6px;border-radius:9999px;background:var(--dfx-accent);animation:dfx-dynamic-island-dot calc(var(--dfx-duration)*.5) ease-in-out infinite}
.dfx-dynamic-island div{display:flex;align-items:center;gap:4px;height:8px}
.dfx-dynamic-island span{width:2px;border-radius:9999px;background:rgba(255,255,255,.7);animation:dfx-dynamic-island-bar calc(var(--dfx-duration)*.364) ease-in-out infinite}
@keyframes dfx-dynamic-island{0%,100%{width:80px}50%{width:110px}}
@keyframes dfx-dynamic-island-dot{0%,100%{opacity:.4}50%{opacity:1}}
@keyframes dfx-dynamic-island-bar{0%,100%{height:3px}50%{height:8px}}
"#;

/// The iOS pill that widens and narrows with a live-activity indicator inside.
#[component]
pub fn DynamicIsland(
    /// Pill colour. Any CSS colour.
    #[props(default = "#09090b".to_string())]
    color: String,
    /// Colour of the status dot.
    #[props(default = "#10b981".to_string())]
    accent_color: String,
    /// Length of one full breath, in seconds.
    #[props(default = 2.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("dynamic-island", CSS)}
        div {
            class: "dfx dfx-loader dfx-dynamic-island {class}",
            style: "--dfx-color:{color};--dfx-accent:{accent_color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            b {}
            div {
                for i in 0..3 {
                    span { key: "{i}", style: "animation-delay:{i as f64 * 0.15}s;" }
                }
            }
        }
    }
}
