use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-origami-shape{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-origami-shape span{position:absolute;width:50%;height:50%;background:var(--dfx-color);animation:dfx-origami-shape var(--dfx-duration) ease-in-out infinite}
.dfx-origami-shape span:nth-child(1){top:0;left:0;border-top-left-radius:6px;transform-origin:right center}
.dfx-origami-shape span:nth-child(2){bottom:0;right:0;opacity:.55;border-bottom-right-radius:6px;transform-origin:left center}
@keyframes dfx-origami-shape{0%,100%{transform:perspective(200px) rotateY(0)}50%{transform:perspective(200px) rotateY(180deg)}}
"#;

/// Two paper-like panels folding over their inner edges in unison.
#[component]
pub fn OrigamiShape(
    /// Width and height of the loader, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Panel colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one fold-and-unfold, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("origami-shape", CSS)}
        div {
            class: "dfx dfx-loader dfx-origami-shape {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
