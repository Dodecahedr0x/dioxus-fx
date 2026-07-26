use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-origami-shape{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-origami-shape span{position:absolute;width:50%;height:50%;background:var(--amt-color);animation:amt-origami-shape var(--amt-duration) ease-in-out infinite}
.amt-origami-shape span:nth-child(1){top:0;left:0;border-top-left-radius:6px;transform-origin:right center}
.amt-origami-shape span:nth-child(2){bottom:0;right:0;opacity:.55;border-bottom-right-radius:6px;transform-origin:left center}
@keyframes amt-origami-shape{0%,100%{transform:perspective(200px) rotateY(0)}50%{transform:perspective(200px) rotateY(180deg)}}
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
        {amt_style!("origami-shape", CSS)}
        div {
            class: "amt amt-loader amt-origami-shape {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
