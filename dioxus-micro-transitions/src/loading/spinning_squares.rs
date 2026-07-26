use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-spinning-squares{position:relative;width:var(--amt-size);height:var(--amt-size)}
.amt-spinning-squares span{position:absolute;width:40%;height:40%;border-radius:3px;background:var(--amt-color);animation:var(--amt-duration) ease-in-out infinite}
.amt-spinning-squares span:nth-child(1){top:0;left:0;animation-name:amt-spinning-squares-a}
.amt-spinning-squares span:nth-child(2){bottom:0;right:0;opacity:.5;animation-name:amt-spinning-squares-b}
@keyframes amt-spinning-squares-a{0%,100%{transform:translate(0,0)}25%{transform:translate(calc(var(--amt-size)*.6),0)}50%{transform:translate(calc(var(--amt-size)*.6),calc(var(--amt-size)*.6))}75%{transform:translate(0,calc(var(--amt-size)*.6))}}
@keyframes amt-spinning-squares-b{0%,100%{transform:translate(0,0)}25%{transform:translate(calc(var(--amt-size)*-.6),0)}50%{transform:translate(calc(var(--amt-size)*-.6),calc(var(--amt-size)*-.6))}75%{transform:translate(0,calc(var(--amt-size)*-.6))}}
"#;

/// Two squares tracing opposite corners of the same box.
#[component]
pub fn SpinningSquares(
    /// Width and height of the box, in pixels.
    #[props(default = 40.0)]
    size: f64,
    /// Square colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for one full lap, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("spinning-squares", CSS)}
        div {
            class: "amt amt-loader amt-spinning-squares {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
