use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-magnetic-dots{display:flex;align-items:center;gap:calc(var(--dfx-size)*.375);filter:url(#dfx-goo-magnetic)}
.dfx-magnetic-dots>svg{position:absolute;width:0;height:0}
.dfx-magnetic-dots span{width:var(--dfx-size);height:var(--dfx-size);border-radius:9999px;background:var(--dfx-color);animation:var(--dfx-duration) ease-in-out infinite}
.dfx-magnetic-dots span:nth-of-type(1){animation-name:dfx-magnetic-dots-a}
.dfx-magnetic-dots span:nth-of-type(2){animation-name:dfx-magnetic-dots-b}
@keyframes dfx-magnetic-dots-a{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--dfx-size)*.5))}}
@keyframes dfx-magnetic-dots-b{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--dfx-size)*-.5))}}
"#;

/// Two dots drawn together until they fuse, then pulled apart again.
#[component]
pub fn MagneticDots(
    /// Diameter of one dot, in pixels.
    #[props(default = 16.0)]
    size: f64,
    /// Dot colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one attract-and-release, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("magnetic-dots", CSS)}
        div {
            class: "dfx dfx-loader dfx-magnetic-dots {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            svg { "aria-hidden": "true",
                defs {
                    filter { id: "dfx-goo-magnetic",
                        feGaussianBlur {
                            "in": "SourceGraphic",
                            std_deviation: "2",
                            result: "blur",
                        }
                        feColorMatrix {
                            "in": "blur",
                            mode: "matrix",
                            values: "1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 15 -7",
                            result: "goo",
                        }
                        feComposite { "in": "SourceGraphic", "in2": "goo", operator: "atop" }
                    }
                }
            }
            span {}
            span {}
        }
    }
}
