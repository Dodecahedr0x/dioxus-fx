use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-pumping-heart{width:var(--dfx-size);height:var(--dfx-size);fill:var(--dfx-color);animation:dfx-pumping-heart var(--dfx-duration) ease-in-out infinite}
@keyframes dfx-pumping-heart{0%,50%,100%{transform:scale(1)}25%,75%{transform:scale(1.25)}}
"#;

/// A heart beating with the characteristic double thump.
#[component]
pub fn PumpingHeart(
    /// Width and height of the heart, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Fill colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one beat cycle, in seconds.
    #[props(default = 1.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("pumping-heart", CSS)}
        svg {
            class: "dfx dfx-loader dfx-pumping-heart {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            view_box: "0 0 24 24",
            role: "status",
            "aria-label": "Loading",
            ..attributes,
            path { d: "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" }
        }
    }
}
