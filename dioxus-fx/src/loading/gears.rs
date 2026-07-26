use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.dfx-gears{display:flex;align-items:flex-start}
.dfx-gears span{border-style:dashed;border-color:var(--dfx-color);border-radius:9999px;animation:dfx-spin-cw linear infinite}
.dfx-gears span:nth-child(1){width:var(--dfx-size);height:var(--dfx-size);border-width:4px;animation-duration:var(--dfx-duration)}
.dfx-gears span:nth-child(2){width:calc(var(--dfx-size)*.75);height:calc(var(--dfx-size)*.75);border-width:3px;opacity:.6;margin-left:-4px;margin-top:calc(var(--dfx-size)*.375);animation-name:dfx-spin-ccw;animation-duration:calc(var(--dfx-duration)*.75)}
@keyframes dfx-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes dfx-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
"#;

/// Two interlocking dashed cogs turning against each other.
#[component]
pub fn Gears(
    /// Diameter of the large cog, in pixels.
    #[props(default = 32.0)]
    size: f64,
    /// Cog colour. Any CSS colour; defaults to the inherited text colour.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Time for the large cog to make one revolution, in seconds.
    #[props(default = 4.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {dfx_style!("gears", CSS)}
        div {
            class: "dfx dfx-loader dfx-gears {class}",
            style: "--dfx-size:{size}px;--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
