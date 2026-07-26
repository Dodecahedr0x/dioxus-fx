//! Pointer-reactive components.
//!
//! Everything here reacts to the pointer over its own element, so these are
//! plain Dioxus event handlers — no JavaScript, no observers. [`TiltCard`] and
//! [`MagneticButton`] additionally need their own size, which they pick up from
//! `onresize` (a `ResizeObserver`, which also fires once on mount).

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[
    GLOW_BUTTON_CSS,
    MAGNETIC_BUTTON_CSS,
    TILT_CARD_CSS,
    CARD_HOVER_CSS,
];

const GLOW_BUTTON_CSS: &str = r#"
.amt-glow-button{position:relative;overflow:hidden;display:inline-flex;align-items:center;justify-content:center;height:2.75rem;padding:0 1.5rem;border:1px solid var(--amt-track);border-radius:.75rem;background:transparent;color:inherit;font:inherit;font-weight:500;font-size:.875rem;cursor:pointer;transition:border-color .2s ease}
.amt-glow-button:hover{border-color:color-mix(in srgb,currentColor 30%,transparent)}
.amt-glow-button__glow{position:absolute;inset:-1px;pointer-events:none;border-radius:.75rem;opacity:0;transition:opacity .3s ease}
.amt-glow-button__label{position:relative;z-index:1}
"#;

/// A button with a glow that follows the pointer across its face.
#[component]
pub fn GlowButton(
    /// Colour of the glow at its centre. Use a translucent colour.
    #[props(default = "rgba(59,130,246,.15)".to_string())]
    glow_color: String,
    /// Radius of the glow, in pixels.
    #[props(default = 120.0)]
    glow_size: f64,
    /// Fired on click.
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut point = use_signal(|| (0.0f64, 0.0f64));
    let mut lit = use_signal(|| false);
    let (x, y) = point();
    let opacity = if lit() { 1 } else { 0 };
    rsx! {
        {amt_style!("glow-button", GLOW_BUTTON_CSS)}
        button {
            class: "amt amt-glow-button {class}",
            onmousemove: move |evt| {
                let p = evt.element_coordinates();
                point.set((p.x, p.y));
            },
            onmouseenter: move |_| lit.set(true),
            onmouseleave: move |_| lit.set(false),
            onclick: move |evt| onclick.call(evt),
            ..attributes,
            span {
                class: "amt-glow-button__glow",
                style: "opacity:{opacity};background:radial-gradient({glow_size}px circle at {x}px {y}px,{glow_color},transparent 80%);",
            }
            span { class: "amt-glow-button__label", {children} }
        }
    }
}

const MAGNETIC_BUTTON_CSS: &str = r#"
.amt-magnetic-button{position:relative;display:inline-flex;align-items:center;justify-content:center;height:2.75rem;padding:0 1.5rem;border:0;border-radius:9999px;background:currentColor;font:inherit;font-weight:600;font-size:.875rem;cursor:pointer;user-select:none;transition:transform .35s cubic-bezier(.22,1,.36,1)}
.amt-magnetic-button__label{pointer-events:none;mix-blend-mode:difference;color:#fff}
@media (prefers-reduced-motion:reduce){.amt-magnetic-button{transition:none}}
"#;

/// A button that leans toward the pointer once it comes within range.
#[component]
pub fn MagneticButton(
    /// How close the pointer must get before the button reacts, in pixels,
    /// measured from the button's centre.
    #[props(default = 45.0)]
    range: f64,
    /// How far the button travels, as a fraction of the pointer's offset from
    /// centre.
    #[props(default = 0.35)]
    strength: f64,
    /// Fired on click.
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut size = use_signal(|| (0.0f64, 0.0f64));
    let mut offset = use_signal(|| (0.0f64, 0.0f64));
    let (dx, dy) = offset();
    rsx! {
        {amt_style!("magnetic-button", MAGNETIC_BUTTON_CSS)}
        button {
            class: "amt amt-magnetic-button {class}",
            style: "transform:translate({dx}px,{dy}px);",
            onresize: move |evt| {
                if let Ok(s) = evt.get_border_box_size() {
                    size.set((s.width, s.height));
                }
            },
            onmousemove: move |evt| {
                let (w, h) = size();
                let p = evt.element_coordinates();
                // `element_coordinates` is relative to the top-left corner; the
                // magnet pulls toward the centre.
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
            span { class: "amt-magnetic-button__label", {children} }
        }
    }
}

const TILT_CARD_CSS: &str = r#"
.amt-tilt-card{position:relative;width:100%;max-width:320px;aspect-ratio:4/3;perspective:800px;cursor:pointer}
.amt-tilt-card__face{display:flex;align-items:center;justify-content:center;width:100%;height:100%;padding:1.5rem;border:1px solid var(--amt-track);border-radius:1rem;box-shadow:0 10px 15px -3px rgba(0,0,0,.1);transform-style:preserve-3d;transition:transform .3s cubic-bezier(.22,1,.36,1);user-select:none}
.amt-tilt-card__content{display:flex;flex-direction:column;align-items:center;justify-content:center;width:100%;height:100%;transform:translateZ(40px);transform-style:preserve-3d}
@media (prefers-reduced-motion:reduce){.amt-tilt-card__face{transition:none}}
"#;

/// A card that tilts in three dimensions to face the pointer.
#[component]
pub fn TiltCard(
    /// Maximum tilt away from flat, in degrees.
    #[props(default = 15.0)]
    max_tilt: f64,
    /// Extra classes for the outer element.
    #[props(default)]
    class: String,
    /// Extra classes for the tilting face.
    #[props(default)]
    card_class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut size = use_signal(|| (0.0f64, 0.0f64));
    let mut tilt = use_signal(|| (0.0f64, 0.0f64));
    let (rx, ry) = tilt();
    rsx! {
        {amt_style!("tilt-card", TILT_CARD_CSS)}
        div {
            class: "amt amt-tilt-card {class}",
            onresize: move |evt| {
                if let Ok(s) = evt.get_border_box_size() {
                    size.set((s.width, s.height));
                }
            },
            onmousemove: move |evt| {
                let (w, h) = size();
                if w == 0.0 || h == 0.0 {
                    return;
                }
                let p = evt.element_coordinates();
                // Normalised to -0.5..0.5, then mapped to the tilt range. The
                // vertical axis drives rotateX and is inverted so the card
                // leans toward the pointer rather than away from it.
                let nx = p.x / w - 0.5;
                let ny = p.y / h - 0.5;
                tilt.set((-ny * 2.0 * max_tilt, nx * 2.0 * max_tilt));
            },
            onmouseleave: move |_| tilt.set((0.0, 0.0)),
            ..attributes,
            div {
                class: "amt-tilt-card__face {card_class}",
                style: "transform:rotateX({rx}deg) rotateY({ry}deg);",
                div { class: "amt-tilt-card__content", {children} }
            }
        }
    }
}

const CARD_HOVER_CSS: &str = r#"
.amt-card-hover{display:grid;grid-template-columns:repeat(var(--amt-columns),minmax(0,1fr));gap:1rem;width:100%}
.amt-card-hover__link{position:relative;display:block;padding:.5rem;height:100%;text-decoration:none;color:inherit}
.amt-card-hover__hl{position:absolute;inset:0;z-index:0;border-radius:1.5rem;background:var(--amt-highlight);opacity:0;transition:opacity .15s ease}
.amt-card-hover__link.amt-active .amt-card-hover__hl{opacity:1}
.amt-card-hover__card{position:relative;z-index:1;overflow:hidden;height:100%;padding:1.25rem;border:1px solid var(--amt-track);border-radius:1rem;transition:border-color .2s ease}
.amt-card-hover__link.amt-active .amt-card-hover__card{border-color:color-mix(in srgb,currentColor 25%,transparent)}
.amt-card-hover__card h4{margin:.5rem 0 0;font-weight:700;letter-spacing:.01em}
.amt-card-hover__card p{margin:.5rem 0 0;font-size:.875rem;line-height:1.6;opacity:.65}
@media (max-width:768px){.amt-card-hover{grid-template-columns:1fr}}
"#;

/// One card in a [`CardHover`] grid.
#[derive(Clone, PartialEq, Debug)]
pub struct CardHoverItem {
    /// Card heading.
    pub title: String,
    /// Card body copy.
    pub description: String,
    /// Where the card links to. `None` renders a non-navigating card.
    pub link: Option<String>,
}

impl CardHoverItem {
    /// Build a card that does not link anywhere.
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            link: None,
        }
    }

    /// Point the card at a URL.
    pub fn with_link(mut self, link: impl Into<String>) -> Self {
        self.link = Some(link.into());
        self
    }
}

/// A grid of cards where a soft highlight slides in behind whichever is hovered.
#[component]
pub fn CardHover(
    /// The cards to lay out.
    items: Vec<CardHoverItem>,
    /// How many columns to use above the mobile breakpoint.
    #[props(default = 3)]
    columns: usize,
    /// Colour of the highlight behind the hovered card.
    #[props(default = "rgba(128,128,128,.18)".to_string())]
    highlight_color: String,
    /// Extra classes for the grid.
    #[props(default)]
    class: String,
    /// Extra classes for each card.
    #[props(default)]
    card_class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut hovered = use_signal(|| None::<usize>);
    rsx! {
        {amt_style!("card-hover", CARD_HOVER_CSS)}
        div {
            class: "amt amt-card-hover {class}",
            style: "--amt-columns:{columns};--amt-highlight:{highlight_color};",
            ..attributes,
            for (i , item) in items.iter().enumerate() {
                a {
                    key: "{i}",
                    class: if hovered() == Some(i) { "amt-card-hover__link amt-active" } else { "amt-card-hover__link" },
                    href: item.link.clone().unwrap_or_else(|| "#".to_string()),
                    onmouseenter: move |_| hovered.set(Some(i)),
                    onmouseleave: move |_| hovered.set(None),
                    span { class: "amt-card-hover__hl" }
                    div { class: "amt-card-hover__card {card_class}",
                        h4 { "{item.title}" }
                        p { "{item.description}" }
                    }
                }
            }
        }
    }
}
