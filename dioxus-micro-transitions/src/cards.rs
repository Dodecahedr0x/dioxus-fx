//! The Amicro card layouts: hover-fanned stacks and index-driven carousels.
//!
//! [`CardSpread`] covers the nine fan layouts. Every card's resting and fanned
//! transform is computed in Rust and handed to CSS as custom properties, so the
//! whole interaction is one `:hover` rule — no state and no JavaScript.
//!
//! [`CardCarousel`], [`CardCoverFlow`] and [`CardTimeMachine`] track an active
//! index, so they hold a signal. Each has a `mono` flag matching the upstream
//! `-mono` variants.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_micro_transitions::cards::{CardSpread, CardSpreadLayout};
//!
//! fn Deck() -> Element {
//!     rsx! {
//!         CardSpread { layout: CardSpreadLayout::Arc5 }
//!     }
//! }
//! ```

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[
    CARD_SPREAD_CSS,
    CARD_CAROUSEL_CSS,
    CARD_COVER_FLOW_CSS,
    CARD_TIME_MACHINE_CSS,
];

/// How a [`CardSpread`] fans its cards out on hover.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CardSpreadLayout {
    /// Five cards in a tight curved arc.
    #[default]
    Arc5,
    /// Seven cards in a wider arc.
    Arc7,
    /// Five cards in a shallow arc spread far to the sides.
    LongArc5,
    /// Five cards sliding apart horizontally, no rotation.
    LinearSpread,
    /// Five cards fanning from a fixed bottom-left anchor.
    CornerFan,
    /// Five cards thrown wide with a hand-stamped tilt.
    StampArc,
    /// Five cards climbing away from the stack in a staircase.
    CascadeStagger,
    /// Five cards scattered at irregular angles.
    ScatterSpread,
    /// Five cards fanning like a hand of playing cards.
    WheelFan,
}

/// Where one card sits when the stack is fanned out.
struct Placement {
    x: f64,
    y: f64,
    rotate: f64,
    scale: f64,
    z: i32,
}

impl CardSpreadLayout {
    /// How many cards this layout draws.
    pub fn count(self) -> usize {
        match self {
            Self::Arc7 => 7,
            _ => 5,
        }
    }

    /// Transform for card `i` in the fanned-out state.
    ///
    /// Ported from the upstream generator; the numbers are its defaults.
    fn placement(self, i: usize) -> Placement {
        let n = self.count();
        let center = (n / 2) as f64;
        let dist = i as f64 - center;
        let adist = dist.abs();
        // Cards nearer the middle stack on top.
        let z = (n as i32 + 1) / 2 + 1 - adist as i32;
        let bump = if dist == 0.0 { 1.05 } else { 1.0 };

        match self {
            Self::Arc5 => {
                let y_offset = 10.0;
                let y = match adist as i32 {
                    2 => y_offset,
                    1 => -0.2 * y_offset,
                    _ => -y_offset,
                };
                Placement {
                    x: dist * 35.0,
                    y,
                    rotate: dist * 15.0,
                    scale: bump,
                    z,
                }
            }
            Self::Arc7 => {
                let y_offset = 30.0;
                let y = match adist as i32 {
                    3 => y_offset,
                    2 => 0.33 * y_offset,
                    1 => -0.17 * y_offset,
                    _ => -0.5 * y_offset,
                };
                Placement {
                    x: dist * (110.0 / 3.0),
                    y,
                    rotate: dist * 15.0,
                    scale: bump,
                    z,
                }
            }
            Self::LongArc5 => {
                let y_offset = 20.0;
                let y = match adist as i32 {
                    2 => y_offset,
                    1 => 0.25 * y_offset,
                    _ => -0.25 * y_offset,
                };
                Placement {
                    x: dist * 70.0,
                    y,
                    rotate: dist * 7.5,
                    scale: bump,
                    z,
                }
            }
            Self::LinearSpread => Placement {
                x: dist * 45.0,
                y: 0.0,
                rotate: 0.0,
                scale: bump,
                z,
            },
            Self::CornerFan => {
                // Fans from a fixed anchor, so the angle is absolute rather
                // than measured from the middle card.
                let ratio = i as f64 / (n - 1) as f64;
                Placement {
                    x: 0.0,
                    y: 0.0,
                    rotate: -10.0 + ratio * 40.0,
                    scale: if i == 2 { 1.03 } else { 1.0 },
                    z: n as i32 - i as i32,
                }
            }
            Self::StampArc => {
                let (arc, spread, y_offset) = (25.0, 180.0, 40.0);
                let (rotate, x, y) = match i {
                    0 => (-arc, -spread, y_offset),
                    1 => (-0.48 * arc, -0.5 * spread, 0.25 * y_offset),
                    2 => (0.0, 0.0, -0.25 * y_offset),
                    3 => (0.48 * arc, 0.5 * spread, 0.25 * y_offset),
                    _ => (arc, spread, y_offset),
                };
                Placement {
                    x,
                    y,
                    rotate,
                    scale: bump,
                    z,
                }
            }
            Self::CascadeStagger => Placement {
                x: dist * 14.0,
                y: dist * -28.0 - 14.0,
                rotate: dist * 6.0,
                scale: if dist == 0.0 { 1.05 } else { 0.98 },
                z: 5 - adist as i32,
            },
            Self::ScatterSpread => {
                const OFFSETS: [(f64, f64, f64); 5] = [
                    (-75.0, 15.0, -14.0),
                    (-35.0, -15.0, -6.0),
                    (0.0, -30.0, 2.0),
                    (35.0, -10.0, 8.0),
                    (75.0, 20.0, 15.0),
                ];
                let (x, y, rotate) = OFFSETS[i.min(4)];
                Placement {
                    x,
                    y,
                    rotate,
                    scale: if i == 2 { 1.05 } else { 0.98 },
                    z: 5 - adist as i32,
                }
            }
            Self::WheelFan => {
                let y = match adist as i32 {
                    2 => -8.0,
                    1 => -22.0,
                    _ => -28.0,
                };
                Placement {
                    x: 0.0,
                    y,
                    rotate: dist * 18.0,
                    scale: if dist == 0.0 { 1.05 } else { 0.98 },
                    z: 5 - adist as i32,
                }
            }
        }
    }

    /// Resting offset, for the one layout that does not start perfectly stacked.
    fn resting_y(self, i: usize) -> f64 {
        match self {
            Self::CascadeStagger => (i as f64 - 2.0) * 2.0,
            _ => 0.0,
        }
    }
}

const CARD_SPREAD_CSS: &str = r#"
.amt-card-spread{position:relative;display:flex;align-items:center;justify-content:center;width:8rem;height:11rem;cursor:pointer}
.amt-card-spread span{position:absolute;inset:0;border:1px solid rgba(255,255,255,.05);border-radius:1rem;background:var(--amt-card);box-shadow:0 4px 10px -2px rgba(0,0,0,.15);transform-origin:50% 100%;transform:translateY(var(--amt-rest-y));transition:transform var(--amt-duration) cubic-bezier(.34,1.4,.64,1)}
.amt-card-spread:hover span{transform:translate(var(--amt-tx),var(--amt-ty)) rotate(var(--amt-rot)) scale(var(--amt-scale))}
@media (prefers-reduced-motion:reduce){.amt-card-spread span{transition:none}}
"#;

/// A stack of blank cards that fans out on hover.
///
/// The cards are deliberately empty — they are a layout primitive. Style them
/// through `card_class`, or set `card_color` for a flat fill.
#[component]
pub fn CardSpread(
    /// Which fan to use.
    #[props(default)]
    layout: CardSpreadLayout,
    /// Card fill. Any CSS colour.
    #[props(default = "#262626".to_string())]
    card_color: String,
    /// How far the cards travel, as a multiple of the layout's defaults.
    #[props(default = 1.0)]
    hover_intensity: f64,
    /// Length of the fan-out, in seconds.
    #[props(default = 0.5)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    /// Extra classes for each card.
    #[props(default)]
    card_class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("card-spread", CARD_SPREAD_CSS)}
        div {
            class: "amt amt-card-spread {class}",
            style: "--amt-card:{card_color};--amt-duration:{duration}s;",
            ..attributes,
            for i in 0..layout.count() {
                {
                    let p = layout.placement(i);
                    let rest_y = layout.resting_y(i);
                    let (tx, ty) = (p.x * hover_intensity, p.y * hover_intensity);
                    let rot = p.rotate * hover_intensity;
                    rsx! {
                        span {
                            key: "{i}",
                            class: "{card_class}",
                            style: "z-index:{p.z};--amt-rest-y:{rest_y}px;--amt-tx:{tx}px;--amt-ty:{ty}px;--amt-rot:{rot}deg;--amt-scale:{p.scale};",
                        }
                    }
                }
            }
        }
    }
}

/// One slide in a carousel.
#[derive(Clone, PartialEq, Debug)]
pub struct CardItem {
    /// Image URL.
    pub src: String,
    /// Accessible description, also used as the caption where one is shown.
    pub title: String,
    /// Timestamp shown by [`CardTimeMachine`].
    pub date: String,
}

impl CardItem {
    /// Build an item from an image and its description.
    pub fn new(src: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            title: title.into(),
            date: String::new(),
        }
    }

    /// Set the timestamp shown by [`CardTimeMachine`].
    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }
}

/// Dot navigation shared by the carousels.
#[component]
fn Dots(count: usize, active: Signal<usize>) -> Element {
    let mut active = active;
    rsx! {
        div { class: "amt-dots",
            for i in 0..count {
                button {
                    key: "{i}",
                    class: if active() == i { "amt-dots__dot amt-active" } else { "amt-dots__dot" },
                    aria_label: "Go to slide {i + 1}",
                    onclick: move |_| active.set(i),
                }
            }
        }
    }
}

const CARD_CAROUSEL_CSS: &str = r#"
.amt-dots{display:flex;justify-content:center;gap:6px;margin-top:12px}
.amt-dots__dot{height:4px;width:4px;padding:0;border:0;border-radius:9999px;background:currentColor;opacity:.3;cursor:pointer;transition:width .3s ease,opacity .3s ease}
.amt-dots__dot.amt-active{width:16px;opacity:1}
.amt-carousel{overflow:hidden;width:100%}
.amt-carousel__track{display:flex;align-items:center;gap:16px;transform:translateX(calc(var(--amt-active) * -160px));transition:transform .5s cubic-bezier(.22,1,.36,1)}
.amt-carousel__slide{flex:none;width:144px;transform-origin:center;transform:rotate(calc(var(--amt-diff) * 5deg)) scale(var(--amt-scale));transition:transform .5s cubic-bezier(.22,1,.36,1)}
.amt-carousel:hover .amt-carousel__slide{transform:translateY(calc(var(--amt-diff) * 24px)) rotate(calc(var(--amt-diff) * 20deg)) scale(var(--amt-scale-hover))}
.amt-carousel img{display:block;width:100%;aspect-ratio:3/4;object-fit:cover;border-radius:12px;cursor:pointer}
.amt-carousel--mono img{filter:grayscale(1)}
.amt-carousel--mono .amt-carousel__slide.amt-active img{filter:none}
@media (prefers-reduced-motion:reduce){.amt-carousel__track,.amt-carousel__slide{transition:none}}
"#;

/// A strip of cards that tilts into a fan while hovered.
#[component]
pub fn CardCarousel(
    /// The slides to show.
    items: Vec<CardItem>,
    /// Which slide starts active.
    #[props(default = 2)]
    initial_index: usize,
    /// Desaturate every slide but the active one.
    #[props(default = false)]
    mono: bool,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let count = items.len();
    let mut active = use_signal(|| initial_index.min(count.saturating_sub(1)));
    let variant = if mono { "amt-carousel--mono" } else { "" };
    let index = active();
    rsx! {
        {amt_style!("card-carousel", CARD_CAROUSEL_CSS)}
        div {
            class: "amt amt-carousel {variant} {class}",
            style: "--amt-active:{index};",
            ..attributes,
            div { class: "amt-carousel__track",
                for (i , item) in items.iter().enumerate() {
                    {
                        let diff = i as i64 - index as i64;
                        let is_active = diff == 0;
                        let scale = if is_active { 1.05 } else { 0.8 };
                        let scale_hover = if is_active { 1.05 } else { 0.65 };
                        rsx! {
                            div {
                                key: "{i}",
                                class: if is_active { "amt-carousel__slide amt-active" } else { "amt-carousel__slide" },
                                style: "--amt-diff:{diff};--amt-scale:{scale};--amt-scale-hover:{scale_hover};",
                                img {
                                    src: "{item.src}",
                                    alt: "{item.title}",
                                    onclick: move |_| active.set(i),
                                }
                            }
                        }
                    }
                }
            }
            Dots { count, active }
        }
    }
}

const CARD_COVER_FLOW_CSS: &str = r#"
.amt-cover-flow{width:100%}
.amt-cover-flow__stage{position:relative;display:flex;align-items:center;justify-content:center;height:140px;perspective:1000px;transform-style:preserve-3d}
.amt-cover-flow__slide{position:absolute;width:104px;transform:translateX(var(--amt-tx)) rotateY(var(--amt-ry)) scale(var(--amt-scale));opacity:var(--amt-opacity);transition:transform .5s cubic-bezier(.22,1,.36,1),opacity .5s ease}
.amt-cover-flow img{display:block;width:100%;aspect-ratio:3/4;object-fit:cover;border-radius:10px;cursor:pointer}
.amt-cover-flow--mono img{filter:grayscale(1)}
.amt-cover-flow--mono .amt-cover-flow__slide.amt-active img{filter:none}
@media (prefers-reduced-motion:reduce){.amt-cover-flow__slide{transition:none}}
"#;

/// The Cover Flow deck: the active card faces you, its neighbours turn away.
#[component]
pub fn CardCoverFlow(
    /// The slides to show.
    items: Vec<CardItem>,
    /// Which slide starts active.
    #[props(default = 2)]
    initial_index: usize,
    /// Desaturate every slide but the active one.
    #[props(default = false)]
    mono: bool,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let count = items.len();
    let mut active = use_signal(|| initial_index.min(count.saturating_sub(1)));
    let variant = if mono { "amt-cover-flow--mono" } else { "" };
    let index = active();
    rsx! {
        {amt_style!("card-cover-flow", CARD_COVER_FLOW_CSS)}
        div { class: "amt amt-cover-flow {variant} {class}", ..attributes,
            div { class: "amt-cover-flow__stage",
                for (i , item) in items.iter().enumerate() {
                    {
                        let offset = i as i64 - index as i64;
                        let abs = offset.unsigned_abs() as f64;
                        let is_active = offset == 0;
                        let ry = if is_active {
                            0.0
                        } else if offset < 0 {
                            38.0
                        } else {
                            -38.0
                        };
                        let scale = if is_active { 1.1 } else { 1.0 - abs * 0.08 };
                        let opacity = if abs > 2.0 { 0.0 } else { 1.0 - abs * 0.25 };
                        let tx = offset as f64 * 32.0;
                        let z = 100 - abs as i32;
                        rsx! {
                            div {
                                key: "{i}",
                                class: if is_active { "amt-cover-flow__slide amt-active" } else { "amt-cover-flow__slide" },
                                style: "z-index:{z};--amt-tx:{tx}px;--amt-ry:{ry}deg;--amt-scale:{scale};--amt-opacity:{opacity};",
                                img {
                                    src: "{item.src}",
                                    alt: "{item.title}",
                                    onclick: move |_| active.set(i),
                                }
                            }
                        }
                    }
                }
            }
            Dots { count, active }
        }
    }
}

const CARD_TIME_MACHINE_CSS: &str = r#"
.amt-time-machine{display:flex;flex-direction:column;align-items:center;gap:16px;width:100%}
.amt-time-machine__stack{position:relative;display:flex;align-items:center;justify-content:center;overflow:hidden;width:100%;height:200px}
.amt-time-machine__card{position:absolute;width:128px;transform:translateY(var(--amt-ty)) scale(var(--amt-scale));opacity:var(--amt-opacity);transition:transform .5s cubic-bezier(.22,1,.36,1),opacity .5s ease}
.amt-time-machine img{display:block;width:100%;aspect-ratio:3/4;object-fit:cover;border-radius:12px}
.amt-time-machine--mono img{filter:grayscale(1)}
.amt-time-machine--mono .amt-time-machine__card.amt-active img{filter:none}
.amt-time-machine__caption{margin-top:-4px;font-size:11px;opacity:.6}
.amt-time-machine__timeline{display:flex;align-items:flex-end;gap:8px;height:24px}
.amt-time-machine__tick{width:2px;height:10px;padding:0;border:0;border-radius:9999px;background:currentColor;opacity:.3;transition:transform .25s ease,opacity .25s ease}
.amt-time-machine__tick.amt-main{height:18px;cursor:pointer}
.amt-time-machine__tick.amt-main:hover,.amt-time-machine__tick.amt-active{opacity:1;transform:scaleX(1.4) scaleY(1.2)}
@media (prefers-reduced-motion:reduce){.amt-time-machine__card,.amt-time-machine__tick{transition:none}}
"#;

/// A photo stack you scrub through with a timeline underneath.
///
/// Photos already stepped past drop out of the frame, the way Time Machine
/// pushes older windows away. The upstream demo runs the stack through an SVG
/// squircle filter for its corner shape; this port uses `border-radius`.
#[component]
pub fn CardTimeMachine(
    /// The photos, newest first. Give each a `date` to label the timeline.
    items: Vec<CardItem>,
    /// Which photo starts on top.
    #[props(default = 0)]
    initial_index: usize,
    /// Desaturate every photo but the active one.
    #[props(default = false)]
    mono: bool,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let count = items.len();
    let mut active = use_signal(|| initial_index.min(count.saturating_sub(1)));
    let variant = if mono { "amt-time-machine--mono" } else { "" };
    let index = active();
    let caption = items.get(index).map(|item| item.title.clone());
    rsx! {
        {amt_style!("card-time-machine", CARD_TIME_MACHINE_CSS)}
        div { class: "amt amt-time-machine {variant} {class}", ..attributes,
            div { class: "amt-time-machine__stack",
                for (i , item) in items.iter().enumerate() {
                    {
                        let offset = i as i64 - index as i64;
                        let is_past = offset < 0;
                        let y = if is_past { 300.0 } else { -(offset as f64) * 12.0 };
                        let opacity = if is_past {
                            0.0
                        } else {
                            1.0 - offset.unsigned_abs() as f64 * 0.2
                        };
                        let scale = if is_past { 1.3 } else { 1.0 };
                        let z = count as i64 - i as i64;
                        rsx! {
                            div {
                                key: "{i}",
                                class: if offset == 0 { "amt-time-machine__card amt-active" } else { "amt-time-machine__card" },
                                style: "z-index:{z};--amt-ty:{y}px;--amt-opacity:{opacity};--amt-scale:{scale};",
                                img { src: "{item.src}", alt: "{item.title}" }
                            }
                        }
                    }
                }
            }
            if let Some(caption) = caption {
                div { class: "amt-time-machine__caption", "{caption}" }
            }
            div { class: "amt-time-machine__timeline",
                for (i , item) in items.iter().enumerate() {
                    {
                        let label = if item.date.is_empty() {
                            item.title.clone()
                        } else {
                            item.date.clone()
                        };
                        rsx! {
                            button {
                                key: "{i}",
                                class: if index == i { "amt-time-machine__tick amt-main amt-active" } else { "amt-time-machine__tick amt-main" },
                                aria_label: "{label}",
                                onmouseenter: move |_| active.set(i),
                                onclick: move |_| active.set(i),
                            }
                            // Two unlabelled ticks between each pair, as upstream.
                            if i + 1 < count {
                                span { class: "amt-time-machine__tick", aria_hidden: "true" }
                                span { class: "amt-time-machine__tick", aria_hidden: "true" }
                            }
                        }
                    }
                }
            }
        }
    }
}
