use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-glassmorphic-card{position:relative;overflow:hidden;display:flex;align-items:center;justify-content:center;width:var(--dfx-width);height:var(--dfx-height);border:1px solid rgba(255,255,255,.5);border-radius:16px;background:var(--dfx-track);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px)}
.dfx-glassmorphic-card i{position:absolute;width:60%;height:75%;border-radius:9999px;background:var(--dfx-glow);filter:blur(18px);animation:dfx-glassmorphic-card var(--dfx-duration) ease-in-out infinite}
.dfx-glassmorphic-card span{position:relative;width:20px;height:20px;border:2px solid var(--dfx-color);border-top-color:transparent;border-radius:9999px;animation:dfx-spin-cw 1s linear infinite}
@keyframes dfx-glassmorphic-card{0%,100%{transform:translate(-20px,-10px)}50%{transform:translate(20px,10px)}}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
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
        {dfx_style!("glassmorphic-card", CSS)}
        div {
            class: "dfx dfx-loader dfx-glassmorphic-card {class}",
            style: "--dfx-width:{width};--dfx-height:{height};--dfx-color:{color};--dfx-glow:{glow_color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            i {}
            span {}
        }
    }
}
