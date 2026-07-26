use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-intersecting-rings{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size);perspective:800px}
.amt-intersecting-rings span{position:absolute;width:83%;height:83%;border:2px solid transparent;border-radius:9999px;transform-style:preserve-3d;animation-duration:var(--amt-duration);animation-timing-function:linear;animation-iteration-count:infinite}
.amt-intersecting-rings span:nth-child(1){border-top-color:var(--amt-color);animation-name:amt-intersecting-rings-a}
.amt-intersecting-rings span:nth-child(2){border-bottom-color:var(--amt-color);animation-name:amt-intersecting-rings-b}
@keyframes amt-intersecting-rings-a{from{transform:rotateX(0) rotateY(0)}to{transform:rotateX(360deg) rotateY(180deg)}}
@keyframes amt-intersecting-rings-b{from{transform:rotateX(0) rotateY(0)}to{transform:rotateX(180deg) rotateY(360deg)}}
"#;

/// Two rings tumbling through each other in three dimensions.
#[component]
pub fn IntersectingRings(
    /// Width and height of the loader, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one full tumble, in seconds.
    #[props(default = 2.2)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("intersecting-rings", CSS)}
        div {
            class: "amt amt-loader amt-intersecting-rings {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
