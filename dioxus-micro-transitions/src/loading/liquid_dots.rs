use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-liquid-dots{position:relative;display:flex;align-items:center;justify-content:center;width:calc(var(--amt-size)*2.667);height:calc(var(--amt-size)*1.333);filter:url(#amt-goo-liquid)}
.amt-liquid-dots>svg{position:absolute;width:0;height:0}
.amt-liquid-dots span{position:absolute;width:var(--amt-size);height:var(--amt-size);border-radius:9999px;background:var(--amt-color);animation:var(--amt-duration) ease-in-out infinite}
.amt-liquid-dots span:nth-of-type(1){animation-name:amt-liquid-dots-a}
.amt-liquid-dots span:nth-of-type(2){animation-name:amt-liquid-dots-b}
@keyframes amt-liquid-dots-a{0%,100%{transform:translateX(calc(var(--amt-size)*-.667))}50%{transform:translateX(calc(var(--amt-size)*.667))}}
@keyframes amt-liquid-dots-b{0%,100%{transform:translateX(calc(var(--amt-size)*.667))}50%{transform:translateX(calc(var(--amt-size)*-.667))}}
"#;

/// Two blobs that merge and separate as they cross, using an SVG gooey filter.
#[component]
pub fn LiquidDots(
    /// Diameter of one blob, in pixels.
    #[props(default = 24.0)]
    size: f64,
    /// Blob colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full pass, in seconds.
    #[props(default = 2.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("liquid-dots", CSS)}
        div {
            class: "amt amt-loader amt-liquid-dots {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            svg { "aria-hidden": "true",
                defs {
                    filter { id: "amt-goo-liquid",
                        feGaussianBlur {
                            "in": "SourceGraphic",
                            std_deviation: "4",
                            result: "blur",
                        }
                        feColorMatrix {
                            "in": "blur",
                            mode: "matrix",
                            values: "1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 18 -7",
                            result: "goo",
                        }
                        feBlend { "in": "SourceGraphic", "in2": "goo" }
                    }
                }
            }
            span {}
            span {}
        }
    }
}
