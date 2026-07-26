use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-skeleton-loader{display:flex;flex-direction:column;gap:8px;width:100%;max-width:var(--amt-width)}
.amt-skeleton-loader>div{display:flex;align-items:center;gap:8px}
.amt-skeleton-loader i,.amt-skeleton-loader b{background:var(--amt-track);border-radius:9999px;animation:amt-skeleton-loader var(--amt-duration) ease-in-out infinite}
.amt-skeleton-loader i{flex:none;width:32px;height:32px}
.amt-skeleton-loader b{height:12px;width:100%}
.amt-skeleton-loader>b{height:8px}
.amt-skeleton-loader>b:last-child{width:80%}
@keyframes amt-skeleton-loader{0%,100%{opacity:.5}50%{opacity:1}}
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
        {amt_style!("skeleton-loader", CSS)}
        div {
            class: "amt amt-loader amt-skeleton-loader {class}",
            style: "--amt-width:{width};--amt-duration:{duration}s;",
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
