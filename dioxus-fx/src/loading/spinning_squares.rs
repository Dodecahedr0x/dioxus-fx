use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-spinning-squares{position:relative;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-spinning-squares span{position:absolute;width:40%;height:40%;border-radius:3px;background:var(--dfx-color);animation:var(--dfx-duration) ease-in-out infinite}
.dfx-spinning-squares span:nth-child(1){top:0;left:0;animation-name:dfx-spinning-squares-a}
.dfx-spinning-squares span:nth-child(2){bottom:0;right:0;opacity:.5;animation-name:dfx-spinning-squares-b}
@keyframes dfx-spinning-squares-a{0%,100%{transform:translate(0,0)}25%{transform:translate(calc(var(--dfx-size)*.6),0)}50%{transform:translate(calc(var(--dfx-size)*.6),calc(var(--dfx-size)*.6))}75%{transform:translate(0,calc(var(--dfx-size)*.6))}}
@keyframes dfx-spinning-squares-b{0%,100%{transform:translate(0,0)}25%{transform:translate(calc(var(--dfx-size)*-.6),0)}50%{transform:translate(calc(var(--dfx-size)*-.6),calc(var(--dfx-size)*-.6))}75%{transform:translate(0,calc(var(--dfx-size)*-.6))}}
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
        {dfx_style!("spinning-squares", CSS)}
        div {
            class: "dfx dfx-loader dfx-spinning-squares {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
