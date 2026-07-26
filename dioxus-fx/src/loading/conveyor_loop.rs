use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-conveyor-loop{position:relative;overflow:hidden;width:var(--dfx-size);height:calc(var(--dfx-size)*.1875);border-radius:9999px;background:var(--dfx-track)}
.dfx-conveyor-loop div{position:absolute;top:0;display:flex;align-items:center;height:100%;gap:calc(var(--dfx-size)*.125);animation:dfx-conveyor-loop var(--dfx-duration) linear infinite}
.dfx-conveyor-loop span{flex:none;width:calc(var(--dfx-size)*.09375);height:calc(var(--dfx-size)*.09375);border-radius:9999px;background:var(--dfx-color)}
@keyframes dfx-conveyor-loop{from{transform:translateX(0)}to{transform:translateX(calc(var(--dfx-size)*-.5))}}
"#;

/// A track of beads sliding endlessly to the left inside a pill.
#[component]
pub fn ConveyorLoop(
    /// Width of the track, in pixels.
    #[props(default = 64.0)]
    size: f64,
    /// Bead colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the belt to advance one repeat, in seconds.
    #[props(default = 1.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("conveyor-loop", CSS)}
        div {
            class: "dfx dfx-loader dfx-conveyor-loop {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            div {
                for n in 0..8 {
                    span { key: "{n}" }
                }
            }
        }
    }
}
