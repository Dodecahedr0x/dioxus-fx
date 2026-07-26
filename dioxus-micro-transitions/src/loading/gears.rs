use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &str = r#"
.amt-gears{display:flex;align-items:flex-start}
.amt-gears span{border-style:dashed;border-color:var(--amt-color);border-radius:9999px;animation:amt-spin-cw linear infinite}
.amt-gears span:nth-child(1){width:var(--amt-size);height:var(--amt-size);border-width:4px;animation-duration:var(--amt-duration)}
.amt-gears span:nth-child(2){width:calc(var(--amt-size)*.75);height:calc(var(--amt-size)*.75);border-width:3px;opacity:.6;margin-left:-4px;margin-top:calc(var(--amt-size)*.375);animation-name:amt-spin-ccw;animation-duration:calc(var(--amt-duration)*.75)}
@keyframes amt-spin-cw{from{transform:rotate(0)}to{transform:rotate(360deg)}}
@keyframes amt-spin-ccw{from{transform:rotate(0)}to{transform:rotate(-360deg)}}
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
        {amt_style!("gears", CSS)}
        div {
            class: "amt amt-loader amt-gears {class}",
            style: "--amt-size:{size}px;--amt-color:{color};--amt-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            span {}
            span {}
        }
    }
}
