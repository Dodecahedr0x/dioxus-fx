use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-apple-equalizer{display:flex;align-items:flex-end;gap:calc(var(--amt-size)*.125);height:var(--amt-size)}
.amt-apple-equalizer span{width:calc(var(--amt-size)*.1875);background:var(--amt-color);border-radius:2px 2px 0 0;animation:amt-apple-equalizer var(--amt-duration) cubic-bezier(.85,0,.15,1) infinite}
@keyframes amt-apple-equalizer{0%,100%{height:20%}50%{height:100%}}
"#;

/// Four audio-meter bars jumping at offset intervals.
///
/// The upstream component randomises each bar's delay on every render; this
/// port uses a fixed, uneven set so that renders stay deterministic.
#[component]
pub fn AppleEqualizer(
    /// Height of the meter, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Bar colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full cycle, in seconds.
    #[props(default = 0.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    const DELAYS: [f64; 4] = [0.0, 0.28, 0.12, 0.4];
    rsx! {
        {amt_style!("apple-equalizer", CSS)}
        div {
            class: "amt amt-loader amt-apple-equalizer {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            for (i , delay) in DELAYS.iter().enumerate() {
                span { key: "{i}", style: "animation-delay:{delay}s;" }
            }
        }
    }
}
