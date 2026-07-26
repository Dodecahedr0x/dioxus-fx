use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-watch-spinner{position:relative;display:flex;align-items:center;justify-content:center;width:var(--amt-size);height:var(--amt-size)}
.amt-watch-spinner span{position:absolute;border:3px solid transparent;border-top-color:var(--amt-color);border-radius:9999px;animation:amt-watch-spinner ease-in-out infinite}
@keyframes amt-watch-spinner{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// Three nested arcs sweeping at decreasing speeds, like watch complications.
#[component]
pub fn WatchSpinner(
    /// Outer diameter, in pixels.
    #[props(default = 48.0)]
    size: f64,
    /// Arc colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the outermost arc to make one revolution, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("watch-spinner", CSS)}
        div {
            class: "amt amt-loader amt-watch-spinner {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for i in 0..3 {
                {
                    // Rings step inward by 12px at the original 48px size.
                    let d = size * (44.0 - i as f64 * 12.0) / 48.0;
                    rsx! {
                        span {
                            key: "{i}",
                            style: "width:{d}px;height:{d}px;animation-duration:{duration + i as f64 * 0.5}s;",
                        }
                    }
                }
            }
        }
    }
}
