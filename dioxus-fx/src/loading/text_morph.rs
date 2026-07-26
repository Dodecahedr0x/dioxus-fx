use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-text-morph{position:relative;overflow:hidden;display:flex;align-items:center;justify-content:center;width:var(--dfx-width);height:24px;font-weight:500;font-size:.875rem;color:var(--dfx-color)}
.dfx-text-morph span{position:absolute;animation:var(--dfx-duration) ease-in-out infinite}
.dfx-text-morph span:nth-child(1){animation-name:dfx-text-morph-out}
.dfx-text-morph span:nth-child(2){animation-name:dfx-text-morph-in}
@keyframes dfx-text-morph-out{0%{opacity:1;transform:translateY(0)}33%{opacity:0;transform:translateY(-16px)}66%{opacity:0;transform:translateY(16px)}100%{opacity:1;transform:translateY(0)}}
@keyframes dfx-text-morph-in{0%{opacity:0;transform:translateY(16px)}33%{opacity:1;transform:translateY(0)}66%{opacity:1;transform:translateY(0)}100%{opacity:0;transform:translateY(-16px)}}
"#;

/// Two words alternating, each sliding out as the other slides in.
#[component]
pub fn TextMorph(
    /// The word shown first.
    #[props(default = "Loading".to_string())]
    first: String,
    /// The word it alternates with.
    #[props(default = "Wait".to_string())]
    second: String,
    /// Width of the window, as a CSS length.
    #[props(default = "96px".to_string())]
    width: String,
    /// Text colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one full swap cycle, in seconds.
    #[props(default = 3.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("text-morph", CSS)}
        div {
            class: "dfx dfx-loader dfx-text-morph {class}",
            style: "--dfx-width:{width};--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "{first}",
            ..attributes,
            span { aria_hidden: "true", "{first}" }
            span { aria_hidden: "true", "{second}" }
        }
    }
}
