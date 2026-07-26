//! Stories for `dioxus_fx::surface`.
//!
//! Every effect here paints over live markup rather than replacing it, so each
//! preview wraps the same [`Panel`]: a heading, a paragraph and a real link.
//! Select the text and follow the link with the effect running — that is the
//! point of the module.

use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_fx::surface::*;
use dioxus_showcase::prelude::*;

/// The radius the effect wrappers share with the panel they cover.
///
/// The overlay covers the wrapper's box and inherits its `border-radius`, so
/// the wrapper needs one of its own to keep its corners off a rounded card.
const CARD_CSS: &str = ".story-card{border-radius:16px}";

/// The content every surface story layers an effect over.
#[component]
pub fn Panel(children: Element) -> Element {
    rsx! {
        document::Style { href: "story:card", {CARD_CSS} }
        div {
            style: "width:340px;padding:28px 30px;border-radius:16px;\
                    background:linear-gradient(135deg,#312e81,#0ea5e9 55%,#f59e0b);\
                    color:#f8fafc;font:15px/1.6 ui-sans-serif,system-ui,sans-serif;",
            h3 { style: "margin:0 0 8px;font:700 19px/1.2 ui-sans-serif,system-ui,sans-serif;",
                "Still live HTML"
            }
            p { style: "margin:0 0 14px;",
                "Select this sentence, or follow the link — the effect is a layer above it, not a picture of it."
            }
            a {
                href: "https://github.com/DavidHDev/canvas-ui",
                style: "color:#fff;font-weight:600;",
                "Where the idea comes from"
            }
            {children}
        }
    }
}

/// A pane of frost that melts clear wherever the pointer goes.
#[story(title = "Surface/Frost", tags = ["surface"])]
pub fn frost(blur: f64, melt: f64, tint: String, intensity: f64) -> Element {
    rsx! {
        Frost {
            class: "story-card",
            blur: num(blur, 9.0),
            melt: num(melt, 130.0),
            tint: txt(tint, "rgba(214,236,255,.26)"),
            intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// A glass puck that follows the pointer and sharpens what is under it.
#[story(title = "Surface/Lens", tags = ["surface"])]
pub fn lens(size: f64, intensity: f64) -> Element {
    rsx! {
        Lens { class: "story-card", size: num(size, 160.0), intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// Rings that spread from every click and bend the content they cross.
#[story(title = "Surface/Ripple", tags = ["surface"])]
pub fn ripple(size: f64, duration: f64, color: String, intensity: f64) -> Element {
    rsx! {
        Ripple {
            class: "story-card",
            size: num(size, 320.0),
            duration: num(duration, 0.9),
            color: txt(color, "rgba(255,255,255,.55)"),
            intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// A corner that lifts on hover to show the layer underneath.
#[story(title = "Surface/Peel", tags = ["surface"])]
pub fn peel(size: f64) -> Element {
    rsx! {
        Peel {
            class: "story-card",
            size: num(size, 96.0),
            beneath: rsx! {
                div {
                    style: "width:100%;height:100%;display:flex;align-items:flex-end;\
                            justify-content:flex-end;padding:18px 20px;\
                            background:linear-gradient(135deg,#f43f5e,#f59e0b);\
                            color:#fff;font:700 15px/1.2 ui-sans-serif,system-ui,sans-serif;",
                    "Underneath"
                }
            },
            div {
                style: "width:340px;padding:28px 30px;\
                        font:15px/1.6 ui-sans-serif,system-ui,sans-serif;",
                h3 { style: "margin:0 0 8px;font:700 19px/1.2 ui-sans-serif,system-ui,sans-serif;",
                    "Hover this card"
                }
                p { style: "margin:0;", "Its corner lifts to reveal a second layer." }
            }
        }
    }
}

/// Which corner lifts is a prop; here it is the bottom-left one.
#[story(title = "Surface/Peel/BottomLeft", tags = ["surface"])]
pub fn peel_bottom_left(size: f64) -> Element {
    rsx! {
        Peel {
            class: "story-card",
            size: num(size, 96.0),
            corner: PeelCorner::BottomLeft,
            beneath: rsx! {
                div {
                    style: "width:100%;height:100%;display:flex;align-items:flex-start;\
                            justify-content:flex-start;padding:18px 20px;\
                            background:linear-gradient(135deg,#0ea5e9,#22c55e);\
                            color:#fff;font:700 15px/1.2 ui-sans-serif,system-ui,sans-serif;",
                    "Underneath"
                }
            },
            div {
                style: "width:340px;padding:28px 30px;\
                        font:15px/1.6 ui-sans-serif,system-ui,sans-serif;",
                h3 { style: "margin:0 0 8px;font:700 19px/1.2 ui-sans-serif,system-ui,sans-serif;",
                    "Same card, other corner"
                }
                p { style: "margin:0;", "All four corners are available." }
            }
        }
    }
}

/// Worn tape playback: scanlines, chroma bleed, head noise and grain.
#[story(title = "Surface/Vhs", tags = ["surface"])]
pub fn vhs(line: f64, shift: f64, intensity: f64) -> Element {
    rsx! {
        Vhs { class: "story-card", line: num(line, 3.0), shift: num(shift, 1.6), intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// Broadcast glitch bursts, idle in between.
#[story(title = "Surface/Glitch", tags = ["surface"])]
pub fn glitch(period: f64, shift: f64, bands: usize, intensity: f64) -> Element {
    rsx! {
        Glitch {
            class: "story-card",
            period: num(period, 6.0),
            shift: num(shift, 4.0),
            bands: num(bands, 3),
            intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// Embers and heat haze rising over the content.
#[story(title = "Surface/Blaze", tags = ["surface"])]
pub fn blaze(color: String, sparks: usize, duration: f64, intensity: f64) -> Element {
    rsx! {
        Blaze {
            class: "story-card",
            color: txt(color, "#ff7a18"),
            sparks: num(sparks, 14),
            duration: num(duration, 3.2),
            intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// A retro dither screen, the one effect here that never animates.
#[story(title = "Surface/Halftone", tags = ["surface"])]
pub fn halftone(cell: f64, intensity: f64) -> Element {
    rsx! {
        Halftone { class: "story-card", cell: num(cell, 4.0), intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}

/// The same screen with `mono` off, so the dots keep their colour.
#[story(title = "Surface/Halftone/Colour", tags = ["surface"])]
pub fn halftone_colour(cell: f64, intensity: f64) -> Element {
    rsx! {
        Halftone { class: "story-card", cell: num(cell, 4.0), mono: false, intensity: num(intensity, 1.0),
            Panel {}
        }
    }
}
