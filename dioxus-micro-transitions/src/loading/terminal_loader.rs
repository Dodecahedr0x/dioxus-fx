use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-terminal-loader{display:flex;flex-direction:column;justify-content:flex-end;overflow:hidden;width:100%;height:64px;padding:12px;border-radius:6px;background:var(--amt-bg);color:var(--amt-color);font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:10px;line-height:1}
.amt-terminal-loader div{display:flex;align-items:center}
.amt-terminal-loader div:first-child{margin-bottom:4px;opacity:.6}
.amt-terminal-loader em{font-style:normal;margin-left:8px}
.amt-terminal-loader i{font-style:normal;color:var(--amt-accent)}
.amt-terminal-loader span{display:block;margin-left:8px;width:6px;height:10px;background:var(--amt-color);animation:amt-terminal-loader var(--amt-duration) steps(1) infinite}
@keyframes amt-terminal-loader{0%,100%{opacity:1}50%{opacity:0}}
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
        {amt_style!("terminal-loader", CSS)}
        div {
            class: "amt amt-loader amt-terminal-loader {class}",
            style: "--amt-bg:{background};--amt-color:{color};--amt-accent:{accent_color};--amt-duration:{duration}s;",
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
