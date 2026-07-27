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

/// A string of soap bubbles trailing the pointer, refracting the page.
#[story(title = "Surface/Bubble", tags = ["surface"])]
pub fn bubble(
    #[default = 68.0] size: f64,
    #[default = 5] count: usize,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Bubble {
            class: "story-card",
            size,
            count,
            intensity,
            Panel {}
        }
    }
}

/// Drifting mist that the pointer parts.
#[story(title = "Surface/Mist", tags = ["surface"])]
pub fn mist(
    #[default = 7.0] blur: f64,
    #[default = 190.0] part: f64,
    #[default = 24.0] duration: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Mist {
            class: "story-card",
            blur,
            part,
            duration,
            intensity,
            Panel {}
        }
    }
}

/// Rain running down the content and refracting whatever it crosses.
#[story(title = "Surface/Droplets", tags = ["surface"])]
pub fn droplets(
    #[default = 16] count: usize,
    #[default = 13.0] size: f64,
    #[default = 2.6] duration: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Droplets {
            class: "story-card",
            count,
            size,
            duration,
            intensity,
            Panel {}
        }
    }
}

/// Lit tiles that ripple in a diagonal wave around the pointer.
#[story(title = "Surface/Tiles", tags = ["surface"])]
pub fn tiles(
    #[default = 9] columns: usize,
    #[default = 6] rows: usize,
    #[default = 210.0] reach: f64,
    #[default = 2.4] duration: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Tiles {
            class: "story-card",
            columns,
            rows,
            reach,
            duration,
            intensity,
            Panel {}
        }
    }
}

/// Floating hex tiles that shine where the pointer goes.
#[story(title = "Surface/Honeycomb", tags = ["surface"])]
pub fn honeycomb(
    #[default = 7] columns: usize,
    #[default = 5] rows: usize,
    #[default = 5.0] lift: f64,
    #[default = 180.0] glow: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Honeycomb {
            class: "story-card",
            columns,
            rows,
            lift,
            glow,
            intensity,
            Panel {}
        }
    }
}

/// A beam that scans down and reveals the content behind it. Change a control
/// to replay it.
#[story(title = "Surface/Laser", tags = ["surface"])]
pub fn laser(
    #[default = "#f43f5e"] color: String,
    #[default = 1.8] duration: f64,
    #[default = 2.0] thickness: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Laser {
            class: "story-card",
            color,
            duration,
            thickness,
            intensity,
            Panel {}
        }
    }
}

/// The same scan on a loop, wiping and redrawing the content each pass.
#[story(title = "Surface/Laser/Repeat", tags = ["surface"])]
pub fn laser_repeat(
    #[default = "#22d3ee"] color: String,
    #[default = 3.0] duration: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Laser {
            class: "story-card",
            color,
            duration,
            repeat: true,
            intensity,
            Panel {}
        }
    }
}

/// Glass that breaks wherever the content is clicked. Click the card.
#[story(title = "Surface/Shatter", tags = ["surface"])]
pub fn shatter(
    #[default = 18] shards: usize,
    #[default = 74.0] rings: f64,
    #[default = 0.55] duration: f64,
    #[default = "rgba(255,255,255,.62)"] color: String,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Shatter {
            class: "story-card",
            shards,
            rings,
            duration,
            color,
            intensity,
            Panel {}
        }
    }
}

/// Fine grain that resolves back into crisp UI around the pointer.
#[story(title = "Surface/Stipple", tags = ["surface"])]
pub fn stipple(
    #[default = 5.0] cell: f64,
    #[default = 2.2] blur: f64,
    #[default = 150.0] focus: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Stipple {
            class: "story-card",
            cell,
            blur,
            focus,
            intensity,
            Panel {}
        }
    }
}

/// A wash of fluid the pointer drags through the content.
#[story(title = "Surface/Liquid", tags = ["surface"])]
pub fn liquid(
    #[default = 240.0] size: f64,
    #[default = 6] count: usize,
    #[default = 9.0] swirl: f64,
    #[default = 90.0] hue: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Liquid {
            class: "story-card",
            size,
            count,
            swirl,
            hue,
            intensity,
            Panel {}
        }
    }
}

/// Content below a line dissolved into grains that reassemble on scroll. Scroll
/// the preview to settle it.
#[story(title = "Surface/Dissolve", tags = ["surface"])]
pub fn dissolve(
    #[default = 45.0] line: f64,
    #[default = 4.0] cell: f64,
    #[default = 1.6] blur: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Dissolve {
            class: "story-card",
            line,
            cell,
            blur,
            intensity,
            Panel {}
        }
    }
}

/// A block that folds away over a virtual edge as the page scrolls past it.
#[story(title = "Surface/Bend", tags = ["surface"])]
pub fn bend(
    #[default = 34.0] angle: f64,
    #[default = 700.0] perspective: f64,
    #[default = 22.0] zone: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Bend {
            class: "story-card",
            angle,
            perspective,
            zone,
            intensity,
            Panel {}
        }
    }
}

/// The backdrop the shape stories refract, since they fill their outline with
/// whatever is behind them rather than with content of their own.
#[component]
pub fn Backdrop(children: Element) -> Element {
    rsx! {
        div {
            style: "width:340px;height:240px;display:flex;align-items:center;\
                    justify-content:center;border-radius:16px;\
                    background:conic-gradient(from 140deg,#f43f5e,#f59e0b,#22c55e,#0ea5e9,#6366f1,#f43f5e);",
            {children}
        }
    }
}

/// A silhouette turned into floating glass over whatever is behind it.
#[story(title = "Surface/GlassShape", tags = ["surface"])]
pub fn glass_shape(
    #[default = 180.0] size: f64,
    #[default = 5.0] refract: f64,
    #[default = 4.0] split: f64,
    #[default = 10.0] float: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Backdrop {
            GlassShape {
                size,
                refract,
                split,
                float,
                intensity,
            }
        }
    }
}

/// A silhouette screened through a one-bit dither.
#[story(title = "Surface/DitherShape", tags = ["surface"])]
pub fn dither_shape(
    #[default = 180.0] size: f64,
    #[default = 4.0] cell: f64,
    #[default = 8.0] float: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Backdrop {
            DitherShape {
                size,
                cell,
                float,
                intensity,
            }
        }
    }
}

/// A silhouette rebuilt as particles that scatter on hover and spring back.
#[story(title = "Surface/ParticleShape", tags = ["surface"])]
pub fn particle_shape(
    #[default = 180.0] size: f64,
    #[default = 5.0] cell: f64,
    #[default = 1.25] scatter: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Backdrop {
            ParticleShape {
                size,
                cell,
                scatter,
                intensity,
            }
        }
    }
}

/// A lens that leaves the page showing only through character-shaped holes.
#[story(title = "Surface/Ascii", tags = ["surface"])]
pub fn ascii(
    #[default = 15.0] cell: f64,
    #[default = 170.0] lens: f64,
    #[default = "rgba(6,10,20,.92)"] ground: String,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Ascii {
            class: "story-card",
            cell,
            lens,
            ground,
            intensity,
            Panel {}
        }
    }
}

/// Content hung on fabric that breathes, swelling under the pointer.
#[story(title = "Surface/Cloth", tags = ["surface"])]
pub fn cloth(
    #[default = 4.0] thread: f64,
    #[default = 190.0] fold: f64,
    #[default = 0.35] sway: f64,
    #[default = 180.0] reach: f64,
    #[default = 1.0] intensity: f64,
) -> Element {
    rsx! {
        Cloth {
            class: "story-card",
            thread,
            fold,
            sway,
            reach,
            intensity,
            Panel {}
        }
    }
}
