use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-apple-breathe{position:relative;display:flex;align-items:center;justify-content:center;width:var(--dfx-size);height:var(--dfx-size)}
.dfx-apple-breathe span{position:absolute;width:calc(var(--dfx-size)*.292);height:calc(var(--dfx-size)*.292);border-radius:9999px;opacity:.38;animation:dfx-apple-breathe var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-apple-breathe{0%,100%{transform:translate(0,0) scale(1)}50%{transform:translate(var(--dfx-x),var(--dfx-y)) scale(1.75)}}
"#;

/// Six translucent dots that drift outward and swell, then draw back in.
#[component]
pub fn AppleBreathe(
    /// Width and height of the loader, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Colour of the even-indexed dots.
    #[props(default = "#2dd4bf".to_string())]
    color: String,
    /// Colour of the odd-indexed dots.
    #[props(default = "#22d3ee".to_string())]
    accent_color: String,
    /// Length of one full breath, in seconds.
    #[props(default = 3.6)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    // The dots sit on a ring whose radius is 11px at the original 48px size.
    let radius = size * 11.0 / 48.0;
    rsx! {
        {dfx_style!("apple-breathe", CSS)}
        div {
            class: "dfx dfx-loader dfx-apple-breathe {class}",
            style: "--dfx-size:{size}px;--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..6 {
                {
                    let angle = (i as f64 * 60.0).to_radians();
                    let x = angle.cos() * radius;
                    let y = angle.sin() * radius;
                    let fill = if i % 2 == 0 { &color } else { &accent_color };
                    rsx! {
                        span {
                            key: "{i}",
                            style: "--dfx-x:{x}px;--dfx-y:{y}px;background:{fill};",
                        }
                    }
                }
            }
        }
    }
}
