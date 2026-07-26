//! Button hover interactions.
//!
//! Twelve interactions behind one component. Pick a [`ButtonInteraction`], hand
//! [`AnimatedButton`] your own icon markup, and it runs on `:hover` and
//! `:focus-visible` in CSS — no state, no JavaScript.
//! [`ButtonInteraction::Magnetic`] is the one exception: it needs the pointer
//! position, so it uses `onmousemove`.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_fx::buttons::{AnimatedButton, ButtonInteraction};
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
//! [`FocusBlurLinks`] is the odd one out: a row of links rather than a button,
//! where hovering one blurs the rest.

use crate::style::dfx_style;
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
            Self::SlideArrow => "dfx-abtn--slide-arrow",
            Self::Sparkle => "dfx-abtn--sparkle",
            Self::Morph | Self::ColorMorph => "dfx-abtn--morph",
            Self::Pulse => "dfx-abtn--pulse",
            Self::Rotate => "dfx-abtn--rotate",
            Self::Shake => "dfx-abtn--shake",
            Self::Ring => "dfx-abtn--ring",
            Self::Glare => "dfx-abtn--glare",
            Self::TextReveal => "dfx-abtn--text-reveal",
            Self::Magnetic => "dfx-abtn--magnetic",
            Self::ExpandRing => "dfx-abtn--expand-ring",
        }
    }
}

const ANIMATED_BUTTON_CSS: &str = r#"
.dfx-abtn{position:relative;display:inline-flex;align-items:center;justify-content:center;height:36px;min-width:75px;padding:0 24px;border:0;border-radius:40px;background:var(--dfx-bg);color:inherit;font:inherit;font-size:13px;font-weight:500;letter-spacing:-.01em;white-space:nowrap;cursor:pointer;transition:padding .25s cubic-bezier(.22,1,.36,1),background-color .15s ease,transform .25s cubic-bezier(.22,1,.36,1)}
.dfx-abtn:hover,.dfx-abtn:focus-visible{padding:0 28px;background:var(--dfx-bg-hover);transform:scale(1.02)}
.dfx-abtn:active{transform:scale(.96)}
.dfx-abtn__icon{position:relative;flex:none;width:16px;height:16px}
.dfx-abtn__icon svg{display:block;width:16px;height:16px}
.dfx-abtn__i1,.dfx-abtn__i2{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;transition:opacity .18s ease,transform .3s cubic-bezier(.34,1.56,.64,1)}
.dfx-abtn__i2{opacity:0;color:var(--dfx-alt-color)}
.dfx-abtn__label{overflow:hidden;height:18px;margin-left:10px;line-height:18px}
.dfx-abtn__label span{display:block;height:18px;line-height:18px}
.dfx-abtn__label span:nth-child(2){display:none}
.dfx-abtn__roll{transition:transform .3s cubic-bezier(.22,1,.36,1)}
.dfx-abtn__glare,.dfx-abtn__ring,.dfx-abtn__badge,.dfx-abtn__spark{display:none}

.dfx-abtn--slide-arrow .dfx-abtn__icon{display:contents}
.dfx-abtn--slide-arrow .dfx-abtn__i1,.dfx-abtn--slide-arrow .dfx-abtn__i2{position:static;flex:none;width:16px;transition:opacity .2s ease,transform .3s cubic-bezier(.34,1.56,.64,1),width .3s cubic-bezier(.22,1,.36,1),margin .3s cubic-bezier(.22,1,.36,1)}
.dfx-abtn--slide-arrow .dfx-abtn__i1{order:0;margin-right:10px}
.dfx-abtn--slide-arrow .dfx-abtn__label{order:1;margin-left:0}
.dfx-abtn--slide-arrow .dfx-abtn__i2{order:2;width:0;margin-left:0;opacity:0;transform:translateX(10px)}
.dfx-abtn--slide-arrow:hover .dfx-abtn__i1,.dfx-abtn--slide-arrow:focus-visible .dfx-abtn__i1{width:0;margin-right:0;opacity:0;transform:translateX(-10px)}
.dfx-abtn--slide-arrow:hover .dfx-abtn__i2,.dfx-abtn--slide-arrow:focus-visible .dfx-abtn__i2{width:16px;margin-left:10px;opacity:1;transform:none}

.dfx-abtn--sparkle .dfx-abtn__i2{transform:translateY(15px) scale(.8)}
.dfx-abtn--sparkle:hover .dfx-abtn__i1,.dfx-abtn--sparkle:focus-visible .dfx-abtn__i1{opacity:0;transform:translateY(-15px) scale(.8)}
.dfx-abtn--sparkle:hover .dfx-abtn__i2,.dfx-abtn--sparkle:focus-visible .dfx-abtn__i2{opacity:1;transform:none}
.dfx-abtn--sparkle .dfx-abtn__spark{display:block;position:absolute;opacity:0;transform:scale(0) rotate(-45deg);transition:opacity .2s ease,transform .3s cubic-bezier(.34,1.56,.64,1)}
.dfx-abtn--sparkle .dfx-abtn__spark svg{display:block;fill:var(--dfx-spark)}
.dfx-abtn--sparkle .dfx-abtn__spark:nth-of-type(1){top:-12px;right:-8px;transition-delay:.05s}
.dfx-abtn--sparkle .dfx-abtn__spark:nth-of-type(1) svg{width:10px;height:10px}
.dfx-abtn--sparkle .dfx-abtn__spark:nth-of-type(2){top:-4px;left:-12px;transition-delay:.1s}
.dfx-abtn--sparkle .dfx-abtn__spark:nth-of-type(2) svg{width:6px;height:6px}
.dfx-abtn--sparkle:hover .dfx-abtn__spark,.dfx-abtn--sparkle:focus-visible .dfx-abtn__spark{opacity:1;transform:none}

.dfx-abtn--morph .dfx-abtn__i2{transform:scale(.5)}
.dfx-abtn--morph:hover .dfx-abtn__i1,.dfx-abtn--morph:focus-visible .dfx-abtn__i1{opacity:0;transform:scale(.5)}
.dfx-abtn--morph:hover .dfx-abtn__i2,.dfx-abtn--morph:focus-visible .dfx-abtn__i2{opacity:1;transform:none}
.dfx-abtn--morph:hover .dfx-abtn__label span:nth-child(1),.dfx-abtn--morph:focus-visible .dfx-abtn__label span:nth-child(1){display:none}
.dfx-abtn--morph:hover .dfx-abtn__label span:nth-child(2),.dfx-abtn--morph:focus-visible .dfx-abtn__label span:nth-child(2){display:block}

@keyframes dfx-abtn-pulse{0%,100%{transform:scale(1)}50%{transform:scale(1.25)}}
.dfx-abtn--pulse:hover .dfx-abtn__i1,.dfx-abtn--pulse:focus-visible .dfx-abtn__i1{color:var(--dfx-alt-color);animation:dfx-abtn-pulse .4s ease-in-out}

.dfx-abtn--rotate:hover .dfx-abtn__i1,.dfx-abtn--rotate:focus-visible .dfx-abtn__i1{transform:rotate(180deg)}

@keyframes dfx-abtn-shake{0%,100%{transform:translateY(0) rotate(0)}25%{transform:translateY(-2px) rotate(-10deg)}50%{transform:translateY(0) rotate(10deg)}75%{transform:translateY(-2px) rotate(-10deg)}}
.dfx-abtn--shake:hover .dfx-abtn__i1,.dfx-abtn--shake:focus-visible .dfx-abtn__i1{color:var(--dfx-alt-color);animation:dfx-abtn-shake .4s ease-in-out}
.dfx-abtn--shake:hover .dfx-abtn__label,.dfx-abtn--shake:focus-visible .dfx-abtn__label{color:var(--dfx-alt-color)}

.dfx-abtn--ring .dfx-abtn__i2{transform:rotate(-15deg) scale(.8)}
.dfx-abtn--ring:hover .dfx-abtn__i1,.dfx-abtn--ring:focus-visible .dfx-abtn__i1{opacity:0;transform:rotate(15deg) scale(.8)}
.dfx-abtn--ring:hover .dfx-abtn__i2,.dfx-abtn--ring:focus-visible .dfx-abtn__i2{opacity:1;transform:none}
.dfx-abtn--ring .dfx-abtn__badge{display:block;position:absolute;top:0;right:0;width:6px;height:6px;border-radius:9999px;background:#ef4444;transform:scale(0);transition:transform .3s cubic-bezier(.34,1.56,.64,1) .1s}
.dfx-abtn--ring:hover .dfx-abtn__badge,.dfx-abtn--ring:focus-visible .dfx-abtn__badge{transform:none}

@keyframes dfx-abtn-glare{0%{transform:translateX(-150%) skewX(-20deg)}100%{transform:translateX(150%) skewX(-20deg)}}
.dfx-abtn--glare{overflow:hidden}
.dfx-abtn--glare .dfx-abtn__glare{display:block;position:absolute;top:0;bottom:0;z-index:1;width:50px;pointer-events:none;background:linear-gradient(90deg,transparent,var(--dfx-glare),transparent);transform:translateX(-150%) skewX(-20deg)}
.dfx-abtn--glare:hover .dfx-abtn__glare,.dfx-abtn--glare:focus-visible .dfx-abtn__glare{animation:dfx-abtn-glare .85s ease-in-out infinite}

.dfx-abtn--text-reveal .dfx-abtn__label span:nth-child(2){display:block}
.dfx-abtn--text-reveal:hover .dfx-abtn__roll,.dfx-abtn--text-reveal:focus-visible .dfx-abtn__roll{transform:translateY(-18px)}
.dfx-abtn--text-reveal:hover .dfx-abtn__i1,.dfx-abtn--text-reveal:focus-visible .dfx-abtn__i1{transform:rotate(45deg)}

@keyframes dfx-abtn-expand-ring{from{opacity:1;transform:scale(1)}to{opacity:0;transform:scale(1.15)}}
.dfx-abtn--expand-ring .dfx-abtn__ring{display:block;position:absolute;inset:0;pointer-events:none;border:1px solid color-mix(in srgb,currentColor 25%,transparent);border-radius:40px;opacity:0}
.dfx-abtn--expand-ring:hover .dfx-abtn__ring,.dfx-abtn--expand-ring:focus-visible .dfx-abtn__ring{animation:dfx-abtn-expand-ring .6s ease-out}
.dfx-abtn--expand-ring:hover .dfx-abtn__i1,.dfx-abtn--expand-ring:focus-visible .dfx-abtn__i1{transform:scale(1.1)}

.dfx-abtn--magnetic{transition:padding .25s cubic-bezier(.22,1,.36,1),background-color .15s ease,transform .35s cubic-bezier(.22,1,.36,1)}
.dfx-abtn--magnetic:hover,.dfx-abtn--magnetic:focus-visible{transform:translate(var(--dfx-dx,0),var(--dfx-dy,0)) scale(1.02)}

@media (prefers-reduced-motion:reduce){.dfx-abtn,.dfx-abtn *{transition:none!important;animation:none!important}}
"#;

const SPARK_PATH: &str = "M12 2l2.4 7.6H22l-6.2 4.5 2.4 7.6-6.2-4.5-6.2 4.5 2.4-7.6L2 9.6h7.6z";

/// A pill button with one of twelve hover interactions.
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
        {dfx_style!("animated-button", ANIMATED_BUTTON_CSS)}
        button {
            class: "dfx dfx-abtn {variant} {class}",
            style: "--dfx-bg:{background};--dfx-bg-hover:{hover_background};--dfx-alt-color:{alt_color};--dfx-spark:{alt_color};--dfx-glare:{glare_color};--dfx-dx:{dx}px;--dfx-dy:{dy}px;",
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
            span { class: "dfx-abtn__icon",
                span { class: "dfx-abtn__i1", {icon} }
                span { class: "dfx-abtn__i2",
                    {alt_icon}
                    span { class: "dfx-abtn__badge" }
                    span { class: "dfx-abtn__spark",
                        svg { view_box: "0 0 24 24",
                            path { d: SPARK_PATH }
                        }
                    }
                    span { class: "dfx-abtn__spark",
                        svg { view_box: "0 0 24 24",
                            path { d: SPARK_PATH }
                        }
                    }
                }
            }
            span { class: "dfx-abtn__label",
                span { class: "dfx-abtn__roll", "{label}" }
                span { "{hover_label}" }
            }
            span { class: "dfx-abtn__glare" }
            span { class: "dfx-abtn__ring" }
        }
    }
}

const FOCUS_BLUR_CSS: &str = r#"
.dfx-focus-blur{display:inline-flex;align-items:center;gap:1rem;font-size:.875rem}
.dfx-focus-blur a{color:inherit;text-decoration:none;transition:filter .25s ease,opacity .25s ease}
.dfx-focus-blur:hover a{filter:blur(var(--dfx-blur));opacity:.45}
.dfx-focus-blur:hover a:hover,.dfx-focus-blur a:focus-visible{filter:none;opacity:1}
.dfx-focus-blur__bracket{opacity:.4}
@media (prefers-reduced-motion:reduce){.dfx-focus-blur a{transition:none}}
"#;

/// A row of links where hovering one blurs the others.
#[component]
pub fn FocusBlurLinks(
    /// The links, as `(label, href)` pairs.
    items: Vec<(String, String)>,
    /// Wrap the row in square brackets.
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
        {dfx_style!("focus-blur", FOCUS_BLUR_CSS)}
        nav {
            class: "dfx dfx-focus-blur {class}",
            style: "--dfx-blur:{blur};",
            ..attributes,
            if show_brackets {
                span { class: "dfx-focus-blur__bracket", aria_hidden: "true", "[" }
            }
            for (i , (label , href)) in items.iter().enumerate() {
                a { key: "{i}", href: "{href}", "{label}" }
            }
            if show_brackets {
                span { class: "dfx-focus-blur__bracket", aria_hidden: "true", "]" }
            }
        }
    }
}
