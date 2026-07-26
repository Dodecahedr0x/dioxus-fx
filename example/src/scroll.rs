//! Stories for `dioxus_fx::scroll`.
//!
//! These respond to page scrolling, so scroll the showcase window rather than
//! the preview to see them move.

use crate::stage::Swatch;
use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_fx::scroll::*;
use dioxus_showcase::prelude::*;

/// A bar pinned to the top of the viewport that fills as the page scrolls.
#[story(title = "Scroll/ProgressIndicator", tags = ["scroll"])]
pub fn progress_indicator(color: String, height: f64) -> Element {
    rsx! {
        ProgressIndicator { color: txt(color, "#3b82f6"), height: num(height, 4.0) }
        div { style: "font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
            "The bar is fixed to the top of the window. Scroll the page to fill it."
        }
    }
}

/// Reveals its children the first time they scroll into view.
#[story(title = "Scroll/ScrollReveal", tags = ["scroll"])]
pub fn scroll_reveal(duration: f64, y_offset: f64, x_offset: f64, scale: f64) -> Element {
    rsx! {
        ScrollReveal {
            duration: num(duration, 0.6),
            y_offset: num(y_offset, 30.0),
            x_offset: num(x_offset, 0.0),
            scale: num(scale, 0.95),
            Swatch {}
        }
    }
}

/// A scrolling column of copy beside a sticky panel that swaps as you read.
#[story(title = "Scroll/StickyReveal", tags = ["scroll"])]
pub fn sticky_reveal(panel_color: String) -> Element {
    rsx! {
        StickyReveal {
            panel_color: txt(panel_color, "rgba(128,128,128,.08)"),
            items: vec![
                StickyRevealItem::new("Read on", "The panel swaps as each block scrolls past.")
                    .with_visual("First"),
                StickyRevealItem::new(
                        "Keep going",
                        "Driven by an IntersectionObserver, not a scroll listener.",
                    )
                    .with_visual("Second"),
                StickyRevealItem::new("Last one", "No scroll maths in Rust at all.")
                    .with_visual("Third"),
            ],
        }
    }
}
