use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-terminal-loader{display:flex;flex-direction:column;justify-content:flex-end;overflow:hidden;width:100%;height:64px;padding:12px;border-radius:6px;background:var(--dfx-bg);color:var(--dfx-color);font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:10px;line-height:1}
.dfx-terminal-loader div{display:flex;align-items:center}
.dfx-terminal-loader div:first-child{margin-bottom:4px;opacity:.6}
.dfx-terminal-loader em{font-style:normal;margin-left:8px}
.dfx-terminal-loader i{font-style:normal;color:var(--dfx-accent)}
.dfx-terminal-loader span{display:block;margin-left:8px;width:6px;height:10px;background:var(--dfx-color);animation:dfx-terminal-loader var(--dfx-duration) steps(1) infinite}
@keyframes dfx-terminal-loader{0%,100%{opacity:1}50%{opacity:0}}
"#;

/// A miniature terminal window with a blinking prompt cursor.
#[component]
pub fn TerminalLoader(
    /// The command echoed on the first line.
    #[props(default = "loading...".to_string())]
    command: String,
    /// Panel background. Any CSS colour.
    #[props(default = "#18181b".to_string())]
    background: String,
    /// Text and cursor colour. Any CSS colour.
    #[props(default = "#f4f4f5".to_string())]
    color: String,
    /// Colour of the prompt caret on the second line.
    #[props(default = "#34d399".to_string())]
    accent_color: String,
    /// Length of one cursor blink, in seconds.
    #[props(default = 0.8)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("terminal-loader", CSS)}
        div {
            class: "dfx dfx-loader dfx-terminal-loader {class}",
            style: "--dfx-bg:{background};--dfx-color:{color};--dfx-accent:{accent_color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            div {
                "$"
                em { "{command}" }
            }
            div {
                i { ">" }
                span { aria_hidden: "true" }
            }
        }
    }
}
