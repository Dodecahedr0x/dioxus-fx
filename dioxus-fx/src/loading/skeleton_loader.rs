use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-skeleton-loader{display:flex;flex-direction:column;gap:8px;width:100%;max-width:var(--dfx-width)}
.dfx-skeleton-loader>div{display:flex;align-items:center;gap:8px}
.dfx-skeleton-loader i,.dfx-skeleton-loader b{background:var(--dfx-track);border-radius:9999px;animation:dfx-skeleton-loader var(--dfx-duration) ease-in-out infinite}
.dfx-skeleton-loader i{flex:none;width:32px;height:32px}
.dfx-skeleton-loader b{height:12px;width:100%}
.dfx-skeleton-loader>b{height:8px}
.dfx-skeleton-loader>b:last-child{width:80%}
@keyframes dfx-skeleton-loader{0%,100%{opacity:.5}50%{opacity:1}}
"#;

/// An avatar-and-lines placeholder, each row breathing a beat behind the last.
#[component]
pub fn SkeletonLoader(
    /// Maximum width of the block, as a CSS length.
    #[props(default = "120px".to_string())]
    width: String,
    /// Length of one breath, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("skeleton-loader", CSS)}
        div {
            class: "dfx dfx-loader dfx-skeleton-loader {class}",
            style: "--dfx-width:{width};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            div {
                i {}
                b { style: "animation-delay:0.2s;" }
            }
            b { style: "animation-delay:0.4s;" }
            b { style: "animation-delay:0.6s;" }
        }
    }
}
