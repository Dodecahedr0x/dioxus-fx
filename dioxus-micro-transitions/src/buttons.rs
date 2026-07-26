//! The Amicro button interaction set.
//!
//! Upstream this is one React component with an `interactionType` switch; the
//! port keeps that shape. Pick a [`ButtonInteraction`], hand [`AnimatedButton`]
//! your own icon markup, and the interaction runs on `:hover` and
//! `:focus-visible` in CSS — no state, no JavaScript.
//! [`ButtonInteraction::Magnetic`] is the one exception: it needs the pointer
//! position, so it uses `onmousemove`.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_micro_transitions::buttons::{AnimatedButton, ButtonInteraction};
//!
//! fn Toolbar() -> Element {
//!     rsx! {
//!         AnimatedButton {
//!             label: "Copy hash",
//!             alt_label: "Copied",
//!             interaction: ButtonInteraction::Morph,
//!             alt_color: "#34d399",
//!             icon: rsx! { svg { view_box: "0 0 24 24" } },
//!             alt_icon: rsx! { svg { view_box: "0 0 24 24" } },
//!         }
//!     }
//! }
//! ```
//!
//! The upstream `focus-blur` entry is a link row rather than a button; it lives
//! here as [`FocusBlurLinks`].

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[ANIMATED_BUTTON_CSS, FOCUS_BLUR_CSS];

/// How an [`AnimatedButton`] reacts to the pointer.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonInteraction {
    /// The leading icon slides out as a trailing icon slides in.
    #[default]
    SlideArrow,
    /// The icon swaps upward and two stars pop out around it.
    Sparkle,
    /// The icon scales out and the alternate icon scales in.
    Morph,
    /// Like [`Morph`](Self::Morph), for swaps that only change colour.
    ColorMorph,
    /// The icon beats once.
    Pulse,
    /// The icon turns a half circle.
    Rotate,
    /// The icon shakes side to side.
    Shake,
    /// The icon tilts out, the alternate tilts in, and a badge pops on.
    Ring,
    /// A highlight sweeps across the button, repeating while hovered.
    Glare,
    /// The label rolls up to reveal a second copy of itself.
    TextReveal,
    /// The button leans toward the pointer.
    Magnetic,
    /// A ring expands out of the button's edge and fades.
    ExpandRing,
}

impl ButtonInteraction {
    fn class(self) -> &'static str {
        match self {
            Self::SlideArrow => "amt-abtn--slide-arrow",
            Self::Sparkle => "amt-abtn--sparkle",
            Self::Morph | Self::ColorMorph => "amt-abtn--morph",
            Self::Pulse => "amt-abtn--pulse",
            Self::Rotate => "amt-abtn--rotate",
            Self::Shake => "amt-abtn--shake",
            Self::Ring => "amt-abtn--ring",
            Self::Glare => "amt-abtn--glare",
            Self::TextReveal => "amt-abtn--text-reveal",
            Self::Magnetic => "amt-abtn--magnetic",
            Self::ExpandRing => "amt-abtn--expand-ring",
        }
    }
}

const ANIMATED_BUTTON_CSS: &str = r#"
.amt-abtn{position:relative;display:inline-flex;align-items:center;justify-content:center;height:36px;min-width:75px;padding:0 24px;border:0;border-radius:40px;background:var(--amt-bg);color:inherit;font:inherit;font-size:13px;font-weight:500;letter-spacing:-.01em;white-space:nowrap;cursor:pointer;transition:padding .25s cubic-bezier(.22,1,.36,1),background-color .15s ease,transform .25s cubic-bezier(.22,1,.36,1)}
.amt-abtn:hover,.amt-abtn:focus-visible{padding:0 28px;background:var(--amt-bg-hover);transform:scale(1.02)}
.amt-abtn:active{transform:scale(.96)}
.amt-abtn__icon{position:relative;flex:none;width:16px;height:16px}
.amt-abtn__icon svg{display:block;width:16px;height:16px}
.amt-abtn__i1,.amt-abtn__i2{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;transition:opacity .18s ease,transform .3s cubic-bezier(.34,1.56,.64,1)}
.amt-abtn__i2{opacity:0;color:var(--amt-alt-color)}
.amt-abtn__label{overflow:hidden;height:18px;margin-left:10px;line-height:18px}
.amt-abtn__label span{display:block;height:18px;line-height:18px}
.amt-abtn__label span:nth-child(2){display:none}
.amt-abtn__roll{transition:transform .3s cubic-bezier(.22,1,.36,1)}
.amt-abtn__glare,.amt-abtn__ring,.amt-abtn__badge,.amt-abtn__spark{display:none}

.amt-abtn--slide-arrow .amt-abtn__icon{display:contents}
.amt-abtn--slide-arrow .amt-abtn__i1,.amt-abtn--slide-arrow .amt-abtn__i2{position:static;flex:none;width:16px;transition:opacity .2s ease,transform .3s cubic-bezier(.34,1.56,.64,1),width .3s cubic-bezier(.22,1,.36,1),margin .3s cubic-bezier(.22,1,.36,1)}
.amt-abtn--slide-arrow .amt-abtn__i1{order:0;margin-right:10px}
.amt-abtn--slide-arrow .amt-abtn__label{order:1;margin-left:0}
.amt-abtn--slide-arrow .amt-abtn__i2{order:2;width:0;margin-left:0;opacity:0;transform:translateX(10px)}
.amt-abtn--slide-arrow:hover .amt-abtn__i1,.amt-abtn--slide-arrow:focus-visible .amt-abtn__i1{width:0;margin-right:0;opacity:0;transform:translateX(-10px)}
.amt-abtn--slide-arrow:hover .amt-abtn__i2,.amt-abtn--slide-arrow:focus-visible .amt-abtn__i2{width:16px;margin-left:10px;opacity:1;transform:none}

.amt-abtn--sparkle .amt-abtn__i2{transform:translateY(15px) scale(.8)}
.amt-abtn--sparkle:hover .amt-abtn__i1,.amt-abtn--sparkle:focus-visible .amt-abtn__i1{opacity:0;transform:translateY(-15px) scale(.8)}
.amt-abtn--sparkle:hover .amt-abtn__i2,.amt-abtn--sparkle:focus-visible .amt-abtn__i2{opacity:1;transform:none}
.amt-abtn--sparkle .amt-abtn__spark{display:block;position:absolute;opacity:0;transform:scale(0) rotate(-45deg);transition:opacity .2s ease,transform .3s cubic-bezier(.34,1.56,.64,1)}
.amt-abtn--sparkle .amt-abtn__spark svg{display:block;fill:var(--amt-spark)}
.amt-abtn--sparkle .amt-abtn__spark:nth-of-type(1){top:-12px;right:-8px;transition-delay:.05s}
.amt-abtn--sparkle .amt-abtn__spark:nth-of-type(1) svg{width:10px;height:10px}
.amt-abtn--sparkle .amt-abtn__spark:nth-of-type(2){top:-4px;left:-12px;transition-delay:.1s}
.amt-abtn--sparkle .amt-abtn__spark:nth-of-type(2) svg{width:6px;height:6px}
.amt-abtn--sparkle:hover .amt-abtn__spark,.amt-abtn--sparkle:focus-visible .amt-abtn__spark{opacity:1;transform:none}

.amt-abtn--morph .amt-abtn__i2{transform:scale(.5)}
.amt-abtn--morph:hover .amt-abtn__i1,.amt-abtn--morph:focus-visible .amt-abtn__i1{opacity:0;transform:scale(.5)}
.amt-abtn--morph:hover .amt-abtn__i2,.amt-abtn--morph:focus-visible .amt-abtn__i2{opacity:1;transform:none}
.amt-abtn--morph:hover .amt-abtn__label span:nth-child(1),.amt-abtn--morph:focus-visible .amt-abtn__label span:nth-child(1){display:none}
.amt-abtn--morph:hover .amt-abtn__label span:nth-child(2),.amt-abtn--morph:focus-visible .amt-abtn__label span:nth-child(2){display:block}

@keyframes amt-abtn-pulse{0%,100%{transform:scale(1)}50%{transform:scale(1.25)}}
.amt-abtn--pulse:hover .amt-abtn__i1,.amt-abtn--pulse:focus-visible .amt-abtn__i1{color:var(--amt-alt-color);animation:amt-abtn-pulse .4s ease-in-out}

.amt-abtn--rotate:hover .amt-abtn__i1,.amt-abtn--rotate:focus-visible .amt-abtn__i1{transform:rotate(180deg)}

@keyframes amt-abtn-shake{0%,100%{transform:translateY(0) rotate(0)}25%{transform:translateY(-2px) rotate(-10deg)}50%{transform:translateY(0) rotate(10deg)}75%{transform:translateY(-2px) rotate(-10deg)}}
.amt-abtn--shake:hover .amt-abtn__i1,.amt-abtn--shake:focus-visible .amt-abtn__i1{color:var(--amt-alt-color);animation:amt-abtn-shake .4s ease-in-out}
.amt-abtn--shake:hover .amt-abtn__label,.amt-abtn--shake:focus-visible .amt-abtn__label{color:var(--amt-alt-color)}

.amt-abtn--ring .amt-abtn__i2{transform:rotate(-15deg) scale(.8)}
.amt-abtn--ring:hover .amt-abtn__i1,.amt-abtn--ring:focus-visible .amt-abtn__i1{opacity:0;transform:rotate(15deg) scale(.8)}
.amt-abtn--ring:hover .amt-abtn__i2,.amt-abtn--ring:focus-visible .amt-abtn__i2{opacity:1;transform:none}
.amt-abtn--ring .amt-abtn__badge{display:block;position:absolute;top:0;right:0;width:6px;height:6px;border-radius:9999px;background:#ef4444;transform:scale(0);transition:transform .3s cubic-bezier(.34,1.56,.64,1) .1s}
.amt-abtn--ring:hover .amt-abtn__badge,.amt-abtn--ring:focus-visible .amt-abtn__badge{transform:none}

@keyframes amt-abtn-glare{0%{transform:translateX(-150%) skewX(-20deg)}100%{transform:translateX(150%) skewX(-20deg)}}
.amt-abtn--glare{overflow:hidden}
.amt-abtn--glare .amt-abtn__glare{display:block;position:absolute;top:0;bottom:0;z-index:1;width:50px;pointer-events:none;background:linear-gradient(90deg,transparent,var(--amt-glare),transparent);transform:translateX(-150%) skewX(-20deg)}
.amt-abtn--glare:hover .amt-abtn__glare,.amt-abtn--glare:focus-visible .amt-abtn__glare{animation:amt-abtn-glare .85s ease-in-out infinite}

.amt-abtn--text-reveal .amt-abtn__label span:nth-child(2){display:block}
.amt-abtn--text-reveal:hover .amt-abtn__roll,.amt-abtn--text-reveal:focus-visible .amt-abtn__roll{transform:translateY(-18px)}
.amt-abtn--text-reveal:hover .amt-abtn__i1,.amt-abtn--text-reveal:focus-visible .amt-abtn__i1{transform:rotate(45deg)}

@keyframes amt-abtn-expand-ring{from{opacity:1;transform:scale(1)}to{opacity:0;transform:scale(1.15)}}
.amt-abtn--expand-ring .amt-abtn__ring{display:block;position:absolute;inset:0;pointer-events:none;border:1px solid color-mix(in srgb,currentColor 25%,transparent);border-radius:40px;opacity:0}
.amt-abtn--expand-ring:hover .amt-abtn__ring,.amt-abtn--expand-ring:focus-visible .amt-abtn__ring{animation:amt-abtn-expand-ring .6s ease-out}
.amt-abtn--expand-ring:hover .amt-abtn__i1,.amt-abtn--expand-ring:focus-visible .amt-abtn__i1{transform:scale(1.1)}

.amt-abtn--magnetic{transition:padding .25s cubic-bezier(.22,1,.36,1),background-color .15s ease,transform .35s cubic-bezier(.22,1,.36,1)}
.amt-abtn--magnetic:hover,.amt-abtn--magnetic:focus-visible{transform:translate(var(--amt-dx,0),var(--amt-dy,0)) scale(1.02)}

@media (prefers-reduced-motion:reduce){.amt-abtn,.amt-abtn *{transition:none!important;animation:none!important}}
"#;

const SPARK_PATH: &str = "M12 2l2.4 7.6H22l-6.2 4.5 2.4 7.6-6.2-4.5-6.2 4.5 2.4-7.6L2 9.6h7.6z";

/// A pill button with one of the Amicro hover interactions.
///
/// You supply the icons — the crate has no icon-set dependency. Pass `icon` for
/// the resting state and `alt_icon` for the state the interaction swaps to;
/// interactions that never swap ignore `alt_icon`.
#[component]
pub fn AnimatedButton(
    /// Text shown on the button.
    label: String,
    /// Text shown while hovered. Falls back to `label`.
    #[props(default)]
    alt_label: Option<String>,
    /// Which interaction to run.
    #[props(default)]
    interaction: ButtonInteraction,
    /// Markup for the resting icon — typically an `svg`.
    #[props(default = rsx! {})]
    icon: Element,
    /// Markup for the swapped-in icon.
    #[props(default = rsx! {})]
    alt_icon: Element,
    /// Colour applied to the swapped-in icon, and to the icon and label for the
    /// pulse and shake interactions.
    #[props(default = "currentColor".to_string())]
    alt_color: String,
    /// Resting background. Any CSS colour.
    #[props(default = "rgba(128,128,128,.08)".to_string())]
    background: String,
    /// Hovered background. Any CSS colour.
    #[props(default = "rgba(128,128,128,.14)".to_string())]
    hover_background: String,
    /// Colour of the band swept across a [`Glare`](ButtonInteraction::Glare)
    /// button.
    #[props(default = "rgba(255,255,255,.22)".to_string())]
    glare_color: String,
    /// How close the pointer must get before a
    /// [`Magnetic`](ButtonInteraction::Magnetic) button reacts, in pixels.
    #[props(default = 45.0)]
    range: f64,
    /// How far a [`Magnetic`](ButtonInteraction::Magnetic) button travels, as a
    /// fraction of the pointer's offset from centre.
    #[props(default = 0.35)]
    strength: f64,
    /// Fired on click.
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut size = use_signal(|| (0.0f64, 0.0f64));
    let mut offset = use_signal(|| (0.0f64, 0.0f64));
    let (dx, dy) = offset();
    let variant = interaction.class();
    let hover_label = alt_label.unwrap_or_else(|| label.clone());
    rsx! {
        {amt_style!("animated-button", ANIMATED_BUTTON_CSS)}
        button {
            class: "amt amt-abtn {variant} {class}",
            style: "--amt-bg:{background};--amt-bg-hover:{hover_background};--amt-alt-color:{alt_color};--amt-spark:{alt_color};--amt-glare:{glare_color};--amt-dx:{dx}px;--amt-dy:{dy}px;",
            onresize: move |evt| {
                if interaction == ButtonInteraction::Magnetic {
                    if let Ok(s) = evt.get_border_box_size() {
                        size.set((s.width, s.height));
                    }
                }
            },
            onmousemove: move |evt| {
                if interaction != ButtonInteraction::Magnetic {
                    return;
                }
                let (w, h) = size();
                let p = evt.element_coordinates();
                let (cx, cy) = (p.x - w / 2.0, p.y - h / 2.0);
                if cx.hypot(cy) < range {
                    offset.set((cx * strength, cy * strength));
                } else {
                    offset.set((0.0, 0.0));
                }
            },
            onmouseleave: move |_| offset.set((0.0, 0.0)),
            onclick: move |evt| onclick.call(evt),
            ..attributes,
            span { class: "amt-abtn__icon",
                span { class: "amt-abtn__i1", {icon} }
                span { class: "amt-abtn__i2",
                    {alt_icon}
                    span { class: "amt-abtn__badge" }
                    span { class: "amt-abtn__spark",
                        svg { view_box: "0 0 24 24",
                            path { d: SPARK_PATH }
                        }
                    }
                    span { class: "amt-abtn__spark",
                        svg { view_box: "0 0 24 24",
                            path { d: SPARK_PATH }
                        }
                    }
                }
            }
            span { class: "amt-abtn__label",
                span { class: "amt-abtn__roll", "{label}" }
                span { "{hover_label}" }
            }
            span { class: "amt-abtn__glare" }
            span { class: "amt-abtn__ring" }
        }
    }
}

const FOCUS_BLUR_CSS: &str = r#"
.amt-focus-blur{display:inline-flex;align-items:center;gap:1rem;font-size:.875rem}
.amt-focus-blur a{color:inherit;text-decoration:none;transition:filter .25s ease,opacity .25s ease}
.amt-focus-blur:hover a{filter:blur(var(--amt-blur));opacity:.45}
.amt-focus-blur:hover a:hover,.amt-focus-blur a:focus-visible{filter:none;opacity:1}
.amt-focus-blur__bracket{opacity:.4}
@media (prefers-reduced-motion:reduce){.amt-focus-blur a{transition:none}}
"#;

/// A row of links where hovering one blurs the others.
#[component]
pub fn FocusBlurLinks(
    /// The links, as `(label, href)` pairs.
    items: Vec<(String, String)>,
    /// Wrap the row in square brackets, as the upstream demo does.
    #[props(default = true)]
    show_brackets: bool,
    /// Blur radius applied to the unfocused links, as a CSS length.
    #[props(default = "2px".to_string())]
    blur: String,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        {amt_style!("focus-blur", FOCUS_BLUR_CSS)}
        nav {
            class: "amt amt-focus-blur {class}",
            style: "--amt-blur:{blur};",
            ..attributes,
            if show_brackets {
                span { class: "amt-focus-blur__bracket", aria_hidden: "true", "[" }
            }
            for (i , (label , href)) in items.iter().enumerate() {
                a { key: "{i}", href: "{href}", "{label}" }
            }
            if show_brackets {
                span { class: "amt-focus-blur__bracket", aria_hidden: "true", "]" }
            }
        }
    }
}
