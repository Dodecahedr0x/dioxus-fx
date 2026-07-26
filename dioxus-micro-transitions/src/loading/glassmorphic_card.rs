use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-glassmorphic-card{position:relative;overflow:hidden;display:flex;align-items:center;justify-content:center;width:var(--amt-width);height:var(--amt-height);border:1px solid rgba(255,255,255,.5);border-radius:16px;background:var(--amt-track);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px)}
.amt-glassmorphic-card i{position:absolute;width:60%;height:75%;border-radius:9999px;background:var(--amt-glow);filter:blur(18px);animation:amt-glassmorphic-card var(--amt-duration) ease-in-out infinite}
.amt-glassmorphic-card span{position:relative;width:20px;height:20px;border:2px solid var(--amt-color);border-top-color:transparent;border-radius:9999px;animation:amt-spin-cw 1s linear infinite}
@keyframes amt-glassmorphic-card{0%,100%{transform:translate(-20px,-10px)}50%{transform:translate(20px,10px)}}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
"#;

/// A frosted glass panel with a drifting glow behind a small spinner.
#[component]
pub fn GlassmorphicCard(
    /// Width of the panel, as a CSS length.
    #[props(default = "80px".to_string())]
    width: String,
    /// Height of the panel, as a CSS length.
    #[props(default = "64px".to_string())]
    height: String,
    /// Spinner colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Colour of the glow drifting behind the glass.
    #[props(default = "rgba(161,161,170,.65)".to_string())]
    glow_color: String,
    /// Time for the glow to complete one drift, in seconds.
    #[props(default = 3.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("glassmorphic-card", CSS)}
        div {
            class: "amt amt-loader amt-glassmorphic-card {class}",
            style: "--amt-width:{width};--amt-height:{height};--amt-color:{color};--amt-glow:{glow_color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            span {}
        }
    }
}
