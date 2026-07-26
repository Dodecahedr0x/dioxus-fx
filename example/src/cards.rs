//! Stories for `dioxus_fx::cards`.
//!
//! `CardSpread` carries all nine layouts, so it gets one story per
//! [`CardSpreadLayout`] variant. Hover a stack to fan it out; click a slide or
//! a dot to move a carousel.

use crate::stage::photos;
use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_fx::cards::*;
use dioxus_showcase::prelude::*;

/// Renders one spread layout with the gallery's card colour.
fn spread(layout: CardSpreadLayout, card_color: String, hover_intensity: f64) -> Element {
    rsx! {
        CardSpread {
            layout,
            card_color: txt(card_color, "#3f3f46"),
            hover_intensity: num(hover_intensity, 1.0),
        }
    }
}

/// Five cards in a tight curved arc.
#[story(title = "Cards/CardSpread/Arc5", tags = ["cards"])]
pub fn card_spread_arc5(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::Arc5, card_color, hover_intensity)
}

/// Seven cards in a wider arc.
#[story(title = "Cards/CardSpread/Arc7", tags = ["cards"])]
pub fn card_spread_arc7(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::Arc7, card_color, hover_intensity)
}

/// Five cards in a shallow arc spread far to the sides.
#[story(title = "Cards/CardSpread/LongArc5", tags = ["cards"])]
pub fn card_spread_long_arc5(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::LongArc5, card_color, hover_intensity)
}

/// Five cards sliding apart horizontally, no rotation.
#[story(title = "Cards/CardSpread/LinearSpread", tags = ["cards"])]
pub fn card_spread_linear(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::LinearSpread, card_color, hover_intensity)
}

/// Five cards fanning from a fixed bottom-left anchor.
#[story(title = "Cards/CardSpread/CornerFan", tags = ["cards"])]
pub fn card_spread_corner_fan(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::CornerFan, card_color, hover_intensity)
}

/// Five cards thrown wide with a hand-stamped tilt.
#[story(title = "Cards/CardSpread/StampArc", tags = ["cards"])]
pub fn card_spread_stamp_arc(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::StampArc, card_color, hover_intensity)
}

/// Five cards climbing away from the stack in a staircase.
#[story(title = "Cards/CardSpread/CascadeStagger", tags = ["cards"])]
pub fn card_spread_cascade_stagger(card_color: String, hover_intensity: f64) -> Element {
    spread(
        CardSpreadLayout::CascadeStagger,
        card_color,
        hover_intensity,
    )
}

/// Five cards scattered at irregular angles.
#[story(title = "Cards/CardSpread/ScatterSpread", tags = ["cards"])]
pub fn card_spread_scatter(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::ScatterSpread, card_color, hover_intensity)
}

/// Five cards fanning like a hand of playing cards.
#[story(title = "Cards/CardSpread/WheelFan", tags = ["cards"])]
pub fn card_spread_wheel_fan(card_color: String, hover_intensity: f64) -> Element {
    spread(CardSpreadLayout::WheelFan, card_color, hover_intensity)
}

/// A row of cards with the active one lifted out of the line.
#[story(title = "Cards/CardCarousel", tags = ["cards"])]
pub fn card_carousel(initial_index: usize) -> Element {
    rsx! {
        CardCarousel { items: photos(), initial_index: num(initial_index, 2) }
    }
}

/// A cover-flow rack: the active card faces you, its neighbours turn away.
#[story(title = "Cards/CardCoverFlow", tags = ["cards"])]
pub fn card_cover_flow(initial_index: usize) -> Element {
    rsx! {
        CardCoverFlow { items: photos(), initial_index: num(initial_index, 2) }
    }
}

/// A receding stack you page back through, one card at a time.
#[story(title = "Cards/CardTimeMachine", tags = ["cards"])]
pub fn card_time_machine(initial_index: usize) -> Element {
    rsx! {
        CardTimeMachine { items: photos(), initial_index }
    }
}
