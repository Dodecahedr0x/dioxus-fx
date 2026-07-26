use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-skeleton{position:relative;overflow:hidden;border-radius:6px;background:var(--dfx-track)}
.dfx-skeleton span{position:absolute;inset:0;width:80%;height:100%;background:linear-gradient(90deg,transparent 0%,var(--dfx-color) 50%,transparent 100%);animation:dfx-skeleton var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-skeleton{from{transform:translateX(-150%)}to{transform:translateX(150%)}}
"#;

/// A bare shimmering placeholder you size yourself.
///
/// Unlike the other loaders this one has no intrinsic dimensions — give it a
/// `class` (or `width`/`height`) so it matches the content it stands in for.
#[component]
pub fn Skeleton(
    /// Width of the block, as a CSS length.
    #[props(default = "100%".to_string())]
    width: String,
    /// Height of the block, as a CSS length.
    #[props(default = "1rem".to_string())]
    height: String,
    /// Colour of the shimmer sweep. Pass a translucent light colour on dark
    /// surfaces and a translucent dark one on light surfaces.
    #[props(default = "rgba(255,255,255,.06)".to_string())]
    color: String,
    /// Time for one sweep, in seconds.
    #[props(default = 1.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("skeleton", CSS)}
        div {
            class: "dfx dfx-loader dfx-skeleton {class}",
            style: "width:{width};height:{height};--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
        }
    }
}
