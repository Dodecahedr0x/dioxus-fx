use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-magnetic-dots{display:flex;align-items:center;gap:calc(var(--amt-size)*.375);filter:url(#amt-goo-magnetic)}
.amt-magnetic-dots>svg{position:absolute;width:0;height:0}
.amt-magnetic-dots span{width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);animation:var(--amt-duration) ease-in-out infinite}
.amt-magnetic-dots span:nth-of-type(1){animation-name:amt-magnetic-dots-a}
.amt-magnetic-dots span:nth-of-type(2){animation-name:amt-magnetic-dots-b}
@keyframes amt-magnetic-dots-a{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--amt-size)*.5))}}
@keyframes amt-magnetic-dots-b{0%,100%{transform:translateX(0)}50%{transform:translateX(calc(var(--amt-size)*-.5))}}
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
        {amt_style!("magnetic-dots", CSS)}
        div {
            class: "amt amt-loader amt-magnetic-dots {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            svg { "aria-hidden": "true",
                defs {
                    filter { id: "amt-goo-magnetic",
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
