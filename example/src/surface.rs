//! Stories for `dioxus_fx::surface`.
//!
//! Every effect here paints over live markup rather than replacing it, so each
//! preview wraps the same [`Panel`]: a heading, a paragraph and a real link.
//! Select the text and follow the link with the effect running — that is the
//! point of the module.

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
pub fn frost(
    #[default = 9.0] blur: f64,
    #[default = 130.0] melt: f64,
    #[default = "rgba(214,236,255,.26)"] tint: String,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Frost {
            class: "story-card",
            blur,
            melt,
            tint,
            intensity,
            Panel {}
        }
    }
}

/// A glass puck that follows the pointer and sharpens what is under it.
#[story(title = "Surface/Lens", tags = ["surface"])]
pub fn lens(#[default = 160.0] size: f64, #[default = 1.0] intensity: f64) -> Element {
    rsx! {
        Lens { class: "story-card", size: size, intensity: intensity,
            Panel {}
        }
    }
}

/// Rings that spread from every click and bend the content they cross.
#[story(title = "Surface/Ripple", tags = ["surface"])]
pub fn ripple(
    #[default = 320.0] size: f64,
    #[default = 0.9] duration: f64,
    #[default = "rgba(255,255,255,.55)"] color: String,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Ripple {
            class: "story-card",
            size,
            duration,
            color,
            intensity,
            Panel {}
        }
    }
}

/// A corner that lifts on hover to show the layer underneath.
#[story(title = "Surface/Peel", tags = ["surface"])]
pub fn peel(#[default = 96.0] size: f64) -> Element {
    rsx! {
        Peel {
            class: "story-card",
            size,
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
pub fn peel_bottom_left(#[default = 96.0] size: f64) -> Element {
    rsx! {
        Peel {
            class: "story-card",
            size,
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
pub fn vhs(
    #[default = 3.0] line: f64,
    #[default = 1.6] shift: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Vhs { class: "story-card", line: line, shift: shift, intensity: intensity,
            Panel {}
        }
    }
}

/// Broadcast glitch bursts, idle in between.
#[story(title = "Surface/Glitch", tags = ["surface"])]
pub fn glitch(
    #[default = 6.0] period: f64,
    #[default = 4.0] shift: f64,
    #[default = 3] bands: usize,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Glitch {
            class: "story-card",
            period,
            shift,
            bands,
            intensity,
            Panel {}
        }
    }
}

/// Embers and heat haze rising over the content.
#[story(title = "Surface/Blaze", tags = ["surface"])]
pub fn blaze(
    #[default = "#ff7a18"] color: String,
    #[default = 14] sparks: usize,
    #[default = 3.2] duration: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Blaze {
            class: "story-card",
            color,
            sparks,
            duration,
            intensity,
            Panel {}
        }
    }
}

/// A retro dither screen, the one effect here that never animates.
#[story(title = "Surface/Halftone", tags = ["surface"])]
pub fn halftone(#[default = 4.0] cell: f64, #[default = 1.0] intensity: f64) -> Element {
    rsx! {
        Halftone { class: "story-card", cell: cell, intensity: intensity,
            Panel {}
        }
    }
}

/// The same screen with `mono` off, so the dots keep their colour.
#[story(title = "Surface/Halftone/Colour", tags = ["surface"])]
pub fn halftone_colour(#[default = 4.0] cell: f64, #[default = 1.0] intensity: f64) -> Element {
    rsx! {
        Halftone { class: "story-card", cell: cell, mono: false, intensity: intensity,
            Panel {}
        }
    }
}
