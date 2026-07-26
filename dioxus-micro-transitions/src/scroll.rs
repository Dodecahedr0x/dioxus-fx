//! Scroll-driven effects.
//!
//! [`ScrollReveal`] and [`StickyReveal`] use Dioxus's `onvisible` event, which
//! is backed by an `IntersectionObserver`. [`ProgressIndicator`] uses CSS
//! scroll-driven animations and needs no JavaScript at all — see its docs for
//! the browser-support caveat.

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[PROGRESS_INDICATOR_CSS, SCROLL_REVEAL_CSS, STICKY_REVEAL_CSS];

const PROGRESS_INDICATOR_CSS: &str = r#"
@keyframes amt-progress-indicator{from{transform:scaleX(0)}to{transform:scaleX(1)}}
.amt-progress-indicator{position:fixed;top:0;left:0;right:0;z-index:9999;height:var(--amt-height);background:var(--amt-color);transform:scaleX(0);transform-origin:0 50%}
@supports (animation-timeline:scroll()){.amt-progress-indicator{animation:amt-progress-indicator linear;animation-timeline:scroll(root block)}}
"#;

/// A bar pinned to the top of the viewport that fills as the page scrolls.
///
/// Driven entirely by CSS `animation-timeline: scroll()`. Browsers without
/// scroll-driven animation support render the bar at zero width rather than
/// showing a misleading full bar, so treat it as a progressive enhancement.
#[component]
pub fn ProgressIndicator(
    /// Bar colour. Any CSS colour.
    #[props(default = "#3b82f6".to_string())]
    color: String,
    /// Bar thickness, in pixels.
    #[props(default = 4.0)]
    height: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("progress-indicator", PROGRESS_INDICATOR_CSS)}
        div {
            class: "amt amt-progress-indicator {class}",
            style: "--amt-color:{color};--amt-height:{height}px;",
            role: "presentation",
            ..attributes,
        }
    }
}

const SCROLL_REVEAL_CSS: &str = r#"
@keyframes amt-scroll-reveal{from{opacity:0;transform:translate(var(--amt-x),var(--amt-y)) scale(var(--amt-scale))}to{opacity:1;transform:none}}
.amt-scroll-reveal{opacity:0}
.amt-scroll-reveal.amt-visible{animation:amt-scroll-reveal var(--amt-duration) cubic-bezier(.16,1,.3,1) both}
@media (prefers-reduced-motion:reduce){.amt-scroll-reveal{opacity:1}.amt-scroll-reveal.amt-visible{animation:none}}
"#;

/// Reveals its children the first time they scroll into view.
///
/// Like the upstream component this only fires once — scrolling back past it
/// does not replay the animation.
#[component]
pub fn ScrollReveal(
    /// Length of the reveal, in seconds.
    #[props(default = 0.6)]
    duration: f64,
    /// How far below its final position the content starts, in pixels.
    #[props(default = 30.0)]
    y_offset: f64,
    /// How far to the side the content starts, in pixels.
    #[props(default = 0.0)]
    x_offset: f64,
    /// Scale factor the content starts at.
    #[props(default = 0.95)]
    scale: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut revealed = use_signal(|| false);
    let state = if revealed() { "amt-visible" } else { "" };
    rsx! {
        {amt_style!("scroll-reveal", SCROLL_REVEAL_CSS)}
        div {
            class: "amt amt-scroll-reveal {state} {class}",
            style: "--amt-duration:{duration}s;--amt-y:{y_offset}px;--amt-x:{x_offset}px;--amt-scale:{scale};",
            onvisible: move |evt| {
                if !revealed() && evt.is_intersecting().unwrap_or(false) {
                    revealed.set(true);
                }
            },
            ..attributes,
            {children}
        }
    }
}

const STICKY_REVEAL_CSS: &str = r#"
.amt-sticky-reveal{position:relative;display:flex;justify-content:space-between;gap:2.5rem;max-width:64rem;margin:0 auto;padding:2.5rem 1rem}
.amt-sticky-reveal__text{display:flex;flex-direction:column;gap:8rem;width:50%;padding:2.5rem 0}
.amt-sticky-reveal__item{display:flex;flex-direction:column;justify-content:center;min-height:50vh;opacity:.3;transition:opacity .4s ease}
.amt-sticky-reveal__item.amt-active{opacity:1}
.amt-sticky-reveal__item h3{margin:0 0 1rem;font-size:1.5rem;font-weight:700}
.amt-sticky-reveal__item p{margin:0;line-height:1.7;opacity:.7}
.amt-sticky-reveal__panel{position:sticky;top:5rem;display:flex;align-items:center;justify-content:center;overflow:hidden;width:50%;height:60vh;border:1px solid var(--amt-track);border-radius:1rem;background:var(--amt-panel)}
.amt-sticky-reveal__card{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;padding:1.5rem;opacity:0;transform:scale(.9);transition:opacity .4s ease,transform .4s ease}
.amt-sticky-reveal__card.amt-active{opacity:1;transform:none}
@media (max-width:768px){.amt-sticky-reveal{flex-direction:column}.amt-sticky-reveal__text,.amt-sticky-reveal__panel{width:100%}}
"#;

/// One entry in a [`StickyReveal`].
#[derive(Clone, PartialEq, Debug)]
pub struct StickyRevealItem {
    /// Heading shown in the scrolling column.
    pub title: String,
    /// Body copy shown under the heading.
    pub description: String,
    /// Text shown on the sticky panel while this item is active.
    pub visual: String,
}

impl StickyRevealItem {
    /// Build an item, defaulting the panel text to the title.
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            visual: title.clone(),
            title,
            description: description.into(),
        }
    }

    /// Replace the text shown on the sticky panel.
    pub fn with_visual(mut self, visual: impl Into<String>) -> Self {
        self.visual = visual.into();
        self
    }
}

/// A scrolling column of copy beside a sticky panel that swaps as you read.
///
/// The active entry is whichever text block is currently intersecting the
/// viewport, so it tracks scrolling without any scroll listener.
#[component]
pub fn StickyReveal(
    /// The entries to page through.
    items: Vec<StickyRevealItem>,
    /// Background of the sticky panel. Any CSS colour.
    #[props(default = "rgba(128,128,128,.08)".to_string())]
    panel_color: String,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut active = use_signal(|| 0usize);
    rsx! {
        {amt_style!("sticky-reveal", STICKY_REVEAL_CSS)}
        div {
            class: "amt amt-sticky-reveal {class}",
            style: "--amt-panel:{panel_color};",
            ..attributes,
            div { class: "amt-sticky-reveal__text",
                for (i , item) in items.iter().enumerate() {
                    div {
                        key: "{i}",
                        class: if active() == i { "amt-sticky-reveal__item amt-active" } else { "amt-sticky-reveal__item" },
                        onvisible: move |evt| {
                            if evt.is_intersecting().unwrap_or(false) {
                                active.set(i);
                            }
                        },
                        h3 { "{item.title}" }
                        p { "{item.description}" }
                    }
                }
            }
            div { class: "amt-sticky-reveal__panel",
                for (i , item) in items.iter().enumerate() {
                    div {
                        key: "{i}",
                        class: if active() == i { "amt-sticky-reveal__card amt-active" } else { "amt-sticky-reveal__card" },
                        "{item.visual}"
                    }
                }
            }
        }
    }
}
