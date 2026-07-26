use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-intersecting-rings{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size);perspective:800px}
.dfx-intersecting-rings span{position:absolute;width:83%;height:83%;border:2px solid transparent;border-radius:9999px;transform-style:preserve-3d;animation-duration:var(--dfx-duration);animation-timing-function:linear;animation-iteration-count:infinite}
.dfx-intersecting-rings span:nth-child(1){border-top-color:var(--dfx-color);animation-name:dfx-intersecting-rings-a}
.dfx-intersecting-rings span:nth-child(2){border-bottom-color:var(--dfx-color);animation-name:dfx-intersecting-rings-b}
@keyframes dfx-intersecting-rings-a{from{transform:rotateX(0) rotateY(0)}to{transform:rotateX(360deg) rotateY(180deg)}}
@keyframes dfx-intersecting-rings-b{from{transform:rotateX(0) rotateY(0)}to{transform:rotateX(180deg) rotateY(360deg)}}
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
        {dfx_style!("intersecting-rings", CSS)}
        div {
            class: "dfx dfx-loader dfx-intersecting-rings {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
