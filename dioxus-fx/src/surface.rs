//! Effects that layer over your existing markup.
//!
//! [Canvas UI](https://github.com/DavidHDev/canvas-ui) makes a good argument:
//! the interesting place for a visual effect is *over live HTML* — text still
//! selectable, links still clickable — rather than inside a canvas that has
//! replaced it. It gets there with WebGL and the experimental html-in-canvas
//! API. This module takes the same idea to where CSS already reaches it:
//! `backdrop-filter` reads whatever is painted behind an element, `mask` and
//! `mix-blend-mode` shape the result, and the content underneath never stops
//! being ordinary DOM.
//!
//! Every component here wraps its children and paints one `pointer-events:none`
//! layer over them, so dropping one around a section you already have changes
//! how it looks and nothing else — no canvas, no shader, no dependency. Each
//! takes an `intensity` between `0` and `1`, because an effect a real design can
//! live with is usually the same effect turned most of the way down.
//!
//! The layer covers the wrapper's own box and inherits its `border-radius`, so
//! an effect going over a rounded card wants that radius on the wrapper too —
//! `Frost { class: "rounded-xl", … }`, or whatever your stylesheet calls it.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_fx::surface::Frost;
//!
//! fn Spoiler() -> Element {
//!     rsx! {
//!         Frost { intensity: 0.8,
//!             p { "Readable again wherever the pointer goes." }
//!         }
//!     }
//! }
//! ```
//!
//! # Browser support
//!
//! Most of these read the content beneath them with `backdrop-filter`. Where
//! that is unsupported each falls back to a plain translucent layer, which is
//! dimmer but never blank. [`Peel`], [`Ripple`] and [`Vhs`] need nothing beyond
//! transforms, masks and keyframes. [`Dissolve`] and [`Bend`] are driven by
//! `animation-timeline: view()`; where that is unsupported each renders its
//! resting state — content whole, block flat — rather than its dramatic one.
//!
//! Three of them are not wrappers: [`GlassShape`], [`DitherShape`] and
//! [`ParticleShape`] take a `src` — an SVG or an image whose alpha is the shape
//! — and fill that outline with the page behind it. They are objects to place
//! rather than effects to wrap something in, which is the same split Canvas UI
//! draws between its components and its `… Object` ones.
//!
//! # What is and is not here
//!
//! Canvas UI reaches its effects through WebGL and the experimental
//! html-in-canvas API, which can do things CSS cannot. Where the mechanism
//! differs the component here says so in its own docs — [`Lens`] resolves where
//! `Magnify` enlarges, [`Bubble`] overlaps where `Bubble` merges as metaballs,
//! [`Bend`] folds a section where `Bend` folds a scanline.
//!
//! All twenty-five of Canvas UI's components have something here now. The last
//! two arrived by giving up on the mechanism rather than the look: [`Ascii`]
//! cannot choose a character per cell's brightness — that needs the rendered
//! pixels read back — so it punches a fixed glyph field out of a sheet and lets
//! the page show through the holes, and [`Cloth`] simulates nothing, reaching
//! for travelling fold shading and a skew where Canvas UI solves a mesh. Both
//! say so in their own docs. Treat the list as covered in spirit, not in
//! method.

use crate::style::dfx_style;
use dioxus::prelude::*;
use std::rc::Rc;

pub(crate) const CSS: &[&str] = &[
    SURFACE_CSS,
    FROST_CSS,
    LENS_CSS,
    RIPPLE_CSS,
    PEEL_CSS,
    VHS_CSS,
    GLITCH_CSS,
    BLAZE_CSS,
    HALFTONE_CSS,
    BUBBLE_CSS,
    MIST_CSS,
    DROPLETS_CSS,
    TILES_CSS,
    HONEYCOMB_CSS,
    LASER_CSS,
    SHATTER_CSS,
    STIPPLE_CSS,
    LIQUID_CSS,
    DISSOLVE_CSS,
    BEND_CSS,
    SHAPE_CSS,
    GLASS_SHAPE_CSS,
    DITHER_SHAPE_CSS,
    PARTICLE_SHAPE_CSS,
    ASCII_CSS,
    CLOTH_CSS,
];

/// The silhouette the [`GlassShape`], [`DitherShape`] and [`ParticleShape`]
/// components cut themselves out of when given no `src` of their own.
///
/// Percent-encoded down to characters that survive a `url('…')` inside an HTML
/// `style` attribute: no spaces, no double quotes, no parentheses.
const DEFAULT_SHAPE: &str = "data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20\
     viewBox=%270%200%20100%20100%27%3E%3Cpath%20d=%27M50%206C68%2030%2086%2046%2086%2062A36%20\
     36%200%200%201%2014%2062C14%2046%2032%2030%2050%206Z%27/%3E%3C/svg%3E";

const SURFACE_CSS: &str = r#"
.dfx-surface{position:relative}
.dfx-surface__content{position:relative}
.dfx-surface__layer{position:absolute;inset:0;z-index:1;overflow:hidden;pointer-events:none;border-radius:inherit}
"#;

/// Element-local pointer coordinates that stay right over child content.
///
/// `element_coordinates` reports the offset from whichever element the event
/// targeted, which for a wrapper around arbitrary markup is normally one of the
/// children rather than the wrapper. These effects need the offset from the
/// wrapper itself, so it is measured when the pointer arrives — once per hover,
/// not once per move — and subtracted from the viewport coordinates of every
/// move after that.
#[derive(Clone, Copy)]
struct LocalPointer {
    node: Signal<Option<Rc<MountedData>>>,
    origin: Signal<(f64, f64)>,
    point: Signal<(f64, f64)>,
    inside: Signal<bool>,
}

/// The wrapper's top-left corner in viewport coordinates.
///
/// Falls back to the origin when the element was never mounted or the renderer
/// cannot measure it, which puts the effect in the corner rather than nowhere.
async fn viewport_origin(node: Option<Rc<MountedData>>) -> (f64, f64) {
    if let Some(node) = node {
        if let Ok(rect) = node.get_client_rect().await {
            return (rect.origin.x, rect.origin.y);
        }
    }
    (0.0, 0.0)
}

fn use_local_pointer() -> LocalPointer {
    LocalPointer {
        node: use_signal(|| None),
        origin: use_signal(|| (0.0, 0.0)),
        point: use_signal(|| (0.0, 0.0)),
        inside: use_signal(|| false),
    }
}

impl LocalPointer {
    /// Keeps the wrapper's handle so its box can be measured later.
    fn mounted(&mut self, data: Rc<MountedData>) {
        self.node.set(Some(data));
    }

    /// Measures the wrapper, then marks the pointer as being over it.
    fn enter(&mut self) {
        let node = self.node.cloned();
        let mut origin = self.origin;
        let mut inside = self.inside;
        spawn(async move {
            origin.set(viewport_origin(node).await);
            inside.set(true);
        });
    }

    /// Records where the pointer is, relative to the wrapper's top-left corner.
    fn moved(&mut self, evt: &MouseEvent) {
        let (ox, oy) = self.origin.cloned();
        let point = evt.client_coordinates();
        self.point.set((point.x - ox, point.y - oy));
    }

    fn leave(&mut self) {
        self.inside.set(false);
    }

    /// `--dfx-x` / `--dfx-y` for the current position, for an inline style.
    fn position(&self) -> String {
        let (x, y) = self.point.cloned();
        format!("--dfx-x:{x}px;--dfx-y:{y}px;")
    }

    fn is_inside(&self) -> bool {
        self.inside.cloned()
    }
}

const FROST_CSS: &str = r#"
@property --dfx-melt{syntax:"<length>";inherits:true;initial-value:0px}
.dfx-frost{transition:--dfx-melt .35s cubic-bezier(.22,1,.36,1)}
.dfx-frost__pane{
background:radial-gradient(140px 100px at 18% 12%,rgba(255,255,255,.20),transparent 70%),radial-gradient(180px 140px at 82% 84%,rgba(255,255,255,.14),transparent 72%),var(--dfx-tint);
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:blur(var(--dfx-blur)) saturate(1.35) brightness(1.04);
backdrop-filter:blur(var(--dfx-blur)) saturate(1.35) brightness(1.04);
-webkit-mask-image:radial-gradient(var(--dfx-melt) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 46%,#000 100%);
mask-image:radial-gradient(var(--dfx-melt) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 46%,#000 100%)
}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-frost__pane{background-color:color-mix(in srgb,var(--dfx-tint) 45%,Canvas)}
}
"#;

/// A pane of frost over its children that melts clear around the pointer.
///
/// The pane blurs and brightens whatever is behind it rather than covering it,
/// so the shape of the content stays legible through the ice. Useful for
/// spoilers, locked sections and previews: the pointer reads one part at a time
/// while the rest stays obscured, and everything under it is still selectable
/// and clickable throughout.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Frost`.
#[component]
pub fn Frost(
    /// How heavily the pane blurs what is behind it, in pixels.
    #[props(default = 9.0)]
    blur: f64,
    /// Radius of the clear circle the pointer melts, in pixels. `0` never melts,
    /// leaving an unbroken pane.
    #[props(default = 130.0)]
    melt: f64,
    /// Colour of the ice. Any CSS colour; use a translucent one.
    #[props(default = "rgba(214,236,255,.26)".to_string())]
    tint: String,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let radius = if pointer.is_inside() {
        melt.max(0.0)
    } else {
        0.0
    };
    let position = pointer.position();
    rsx! {
        {dfx_style!("frost", FROST_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-frost {class}",
            style: "--dfx-blur:{blur}px;--dfx-tint:{tint};--dfx-intensity:{intensity};--dfx-melt:{radius}px;{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-frost__pane", aria_hidden: "true" }
        }
    }
}

const LENS_CSS: &str = r#"
.dfx-lens__glass{
position:absolute;top:0;left:0;width:var(--dfx-size);height:var(--dfx-size);
margin:calc(var(--dfx-size)/-2) 0 0 calc(var(--dfx-size)/-2);
border-radius:9999px;opacity:var(--dfx-shown);
transform:translate(var(--dfx-x),var(--dfx-y));
background:linear-gradient(150deg,rgba(255,255,255,.24),rgba(255,255,255,0) 48%,rgba(255,255,255,.12));
box-shadow:inset 0 1px 0 rgba(255,255,255,.55),inset 0 -10px 20px rgba(255,255,255,.12),0 12px 26px rgba(0,0,0,.20),0 0 0 1px var(--dfx-track);
-webkit-backdrop-filter:saturate(var(--dfx-saturate)) contrast(var(--dfx-contrast)) brightness(1.05);
backdrop-filter:saturate(var(--dfx-saturate)) contrast(var(--dfx-contrast)) brightness(1.05);
transition:opacity .25s ease
}
"#;

/// A glass puck that follows the pointer and sharpens the page under it.
///
/// The puck has no content of its own: the colour inside it is the content
/// behind it, pushed through `saturate` and `contrast`, with a rim highlight
/// and a drop shadow to sell the thickness. Good for drawing an eye across a
/// dense layout — a chart, a table, a screenshot — without dimming the rest.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Glass` and
/// `Magnify`. CSS cannot magnify live DOM, so this lens resolves rather than
/// enlarges.
#[component]
pub fn Lens(
    /// Diameter of the lens, in pixels.
    #[props(default = 160.0)]
    size: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    // Saturation carries most of the effect; contrast follows more gently so a
    // lens at full strength still reads as glass rather than a filter preset.
    let saturate = 1.0 + 0.9 * intensity;
    let contrast = 1.0 + 0.25 * intensity;
    let shown = if pointer.is_inside() { 1 } else { 0 };
    let position = pointer.position();
    rsx! {
        {dfx_style!("lens", LENS_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-lens {class}",
            style: "--dfx-size:{size}px;--dfx-saturate:{saturate};--dfx-contrast:{contrast};--dfx-shown:{shown};{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                span { class: "dfx-lens__glass" }
            }
        }
    }
}

const RIPPLE_CSS: &str = r#"
@keyframes dfx-ripple{
from{transform:translate(var(--dfx-x),var(--dfx-y)) scale(.05);opacity:var(--dfx-intensity)}
to{transform:translate(var(--dfx-x),var(--dfx-y)) scale(1);opacity:0}
}
.dfx-ripple__drop{
position:absolute;top:0;left:0;width:var(--dfx-size);height:var(--dfx-size);
margin:calc(var(--dfx-size)/-2) 0 0 calc(var(--dfx-size)/-2);
border-radius:9999px;
background:radial-gradient(closest-side,transparent 54%,var(--dfx-color) 72%,transparent 100%);
-webkit-backdrop-filter:blur(1.5px) brightness(1.06);
backdrop-filter:blur(1.5px) brightness(1.06);
-webkit-mask-image:radial-gradient(closest-side,transparent 50%,#000 70%,transparent 100%);
mask-image:radial-gradient(closest-side,transparent 50%,#000 70%,transparent 100%);
animation:dfx-ripple var(--dfx-duration) cubic-bezier(.22,1,.36,1) forwards
}
"#;

/// Rings that spread from wherever its children are clicked.
///
/// The ring is an annulus of `backdrop-filter`, so it bends the content it
/// crosses instead of tinting a shape on top of it. The overlay never takes
/// pointer events, so buttons and links inside keep working — the wrapper only
/// watches the clicks on their way past.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Ripple`.
#[component]
pub fn Ripple(
    /// Diameter each ring grows to, in pixels.
    #[props(default = 320.0)]
    size: f64,
    /// How long a ring takes to spread and fade, in seconds.
    #[props(default = 0.9)]
    duration: f64,
    /// Colour of the ring. Any CSS colour.
    #[props(default = "rgba(255,255,255,.55)".to_string())]
    color: String,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    /// Rings live until this many newer ones have started. Their animation
    /// leaves them invisible long before that, so nothing has to time their
    /// removal.
    const KEPT: usize = 6;

    let mut node = use_signal(|| None::<Rc<MountedData>>);
    let mut drops = use_signal(Vec::<(usize, f64, f64)>::new);
    let mut next = use_signal(|| 0usize);
    let intensity = intensity.clamp(0.0, 1.0);
    rsx! {
        {dfx_style!("ripple", RIPPLE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-ripple {class}",
            style: "--dfx-size:{size}px;--dfx-duration:{duration}s;--dfx-color:{color};--dfx-intensity:{intensity};",
            onmounted: move |evt| node.set(Some(evt.data())),
            onclick: move |evt: MouseEvent| {
                let handle = node.cloned();
                let point = evt.client_coordinates();
                spawn(async move {
                    // Measured per click rather than cached: a ring is cheap to
                    // start late and wrong if it starts in the wrong place, and
                    // a touch never sends the `mouseenter` a cache would use.
                    let (ox, oy) = viewport_origin(handle).await;
                    let id = next.cloned();
                    next.set(id + 1);
                    let mut rings = drops.write();
                    rings.push((id, point.x - ox, point.y - oy));
                    if rings.len() > KEPT {
                        rings.remove(0);
                    }
                });
            },
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                for (id , x , y) in drops() {
                    span { key: "{id}", class: "dfx-ripple__drop", style: "--dfx-x:{x}px;--dfx-y:{y}px;" }
                }
            }
        }
    }
}

const PEEL_CSS: &str = r#"
.dfx-peel{overflow:hidden}
.dfx-peel__under{position:absolute;inset:0;z-index:0;overflow:hidden;border-radius:inherit}
.dfx-peel__top{position:relative;z-index:1;background:var(--dfx-face);border-radius:inherit;transition:clip-path .45s cubic-bezier(.22,1,.36,1)}
.dfx-peel__flap{position:absolute;z-index:2;width:var(--dfx-size);height:var(--dfx-size);pointer-events:none;opacity:0;transform:scale(.2);transition:opacity .45s ease,transform .45s cubic-bezier(.22,1,.36,1);filter:drop-shadow(0 4px 10px rgba(0,0,0,.28))}
.dfx-peel__fold{display:block;width:100%;height:100%;background:linear-gradient(var(--dfx-fold),color-mix(in srgb,currentColor 6%,var(--dfx-face)),color-mix(in srgb,currentColor 34%,var(--dfx-face)))}
.dfx-peel:hover .dfx-peel__flap,.dfx-peel:focus-within .dfx-peel__flap{opacity:1;transform:scale(1)}

.dfx-peel--tr .dfx-peel__top{clip-path:polygon(0 0,100% 0,100% 0,100% 100%,0 100%)}
.dfx-peel--tr:hover .dfx-peel__top,.dfx-peel--tr:focus-within .dfx-peel__top{clip-path:polygon(0 0,calc(100% - var(--dfx-size)) 0,100% var(--dfx-size),100% 100%,0 100%)}
.dfx-peel--tr .dfx-peel__flap{top:0;right:0;transform-origin:100% 0}
.dfx-peel--tr .dfx-peel__fold{clip-path:polygon(100% 0,100% 100%,0 0)}

.dfx-peel--tl .dfx-peel__top{clip-path:polygon(0 0,0 0,100% 0,100% 100%,0 100%)}
.dfx-peel--tl:hover .dfx-peel__top,.dfx-peel--tl:focus-within .dfx-peel__top{clip-path:polygon(0 var(--dfx-size),var(--dfx-size) 0,100% 0,100% 100%,0 100%)}
.dfx-peel--tl .dfx-peel__flap{top:0;left:0;transform-origin:0 0}
.dfx-peel--tl .dfx-peel__fold{clip-path:polygon(0 0,100% 0,0 100%)}

.dfx-peel--br .dfx-peel__top{clip-path:polygon(0 0,100% 0,100% 100%,100% 100%,0 100%)}
.dfx-peel--br:hover .dfx-peel__top,.dfx-peel--br:focus-within .dfx-peel__top{clip-path:polygon(0 0,100% 0,100% calc(100% - var(--dfx-size)),calc(100% - var(--dfx-size)) 100%,0 100%)}
.dfx-peel--br .dfx-peel__flap{bottom:0;right:0;transform-origin:100% 100%}
.dfx-peel--br .dfx-peel__fold{clip-path:polygon(100% 0,100% 100%,0 100%)}

.dfx-peel--bl .dfx-peel__top{clip-path:polygon(0 0,100% 0,100% 100%,0 100%,0 100%)}
.dfx-peel--bl:hover .dfx-peel__top,.dfx-peel--bl:focus-within .dfx-peel__top{clip-path:polygon(0 0,100% 0,100% 100%,var(--dfx-size) 100%,0 calc(100% - var(--dfx-size)))}
.dfx-peel--bl .dfx-peel__flap{bottom:0;left:0;transform-origin:0 100%}
.dfx-peel--bl .dfx-peel__fold{clip-path:polygon(0 0,100% 100%,0 100%)}
@media (prefers-reduced-motion:reduce){.dfx-peel__top,.dfx-peel__flap{transition:none}}
"#;

/// Which corner a [`Peel`] lifts from.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PeelCorner {
    /// Lift the top-right corner.
    #[default]
    TopRight,
    /// Lift the top-left corner.
    TopLeft,
    /// Lift the bottom-right corner.
    BottomRight,
    /// Lift the bottom-left corner.
    BottomLeft,
}

impl PeelCorner {
    /// The modifier class carrying this corner's geometry.
    fn class(self) -> &'static str {
        match self {
            Self::TopRight => "dfx-peel--tr",
            Self::TopLeft => "dfx-peel--tl",
            Self::BottomRight => "dfx-peel--br",
            Self::BottomLeft => "dfx-peel--bl",
        }
    }

    /// The direction the folded-back flap shades along, so its light always
    /// falls away from the corner it hinges on.
    fn fold(self) -> &'static str {
        match self {
            Self::TopRight => "225deg",
            Self::TopLeft => "135deg",
            Self::BottomRight => "315deg",
            Self::BottomLeft => "45deg",
        }
    }
}

/// A card whose corner lifts on hover, showing a second layer underneath.
///
/// `beneath` is the layer that gets revealed; `children` are the face that peels
/// back. The fold is a clip-path on the face plus a shaded triangle for the
/// lifted paper, so the face keeps its own background and both layers stay live
/// DOM. Reacts to `:focus-within` as well as `:hover`, so it opens for keyboard
/// users too.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Peel`.
#[component]
pub fn Peel(
    /// The layer revealed under the fold.
    beneath: Element,
    /// Which corner lifts.
    #[props(default)]
    corner: PeelCorner,
    /// How far the corner lifts, in pixels.
    #[props(default = 96.0)]
    size: f64,
    /// Background of the face that peels. It has to be opaque or the layer
    /// underneath shows through everywhere; the default is the page's own
    /// background colour, which follows a light or dark theme on its own.
    #[props(default = "Canvas".to_string())]
    face: String,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let corner_class = corner.class();
    let fold = corner.fold();
    rsx! {
        {dfx_style!("peel", PEEL_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-peel {corner_class} {class}",
            style: "--dfx-size:{size}px;--dfx-face:{face};--dfx-fold:{fold};",
            ..attributes,
            div { class: "dfx-peel__under", {beneath} }
            div { class: "dfx-peel__top", {children} }
            // The shadow lives on the wrapper and the triangle on the child:
            // `clip-path` is applied after `filter`, so a shadow on the clipped
            // element would be cut away with everything else outside the fold.
            span { class: "dfx-peel__flap", aria_hidden: "true",
                span { class: "dfx-peel__fold" }
            }
        }
    }
}

const VHS_CSS: &str = r#"
@keyframes dfx-vhs-scan{from{background-position-y:0}to{background-position-y:var(--dfx-line)}}
@keyframes dfx-vhs-head{from{transform:translateY(-30%)}to{transform:translateY(130%)}}
@keyframes dfx-vhs-grain{from{background-position:0 0}to{background-position:117px 83px}}
@keyframes dfx-vhs-wobble{0%,84%,100%{transform:none}86%{transform:translateX(calc(var(--dfx-shift)*-2))}88%{transform:translateX(calc(var(--dfx-shift)*1.5))}90%{transform:none}94%{transform:translateX(var(--dfx-shift))}96%{transform:none}}
.dfx-vhs .dfx-surface__content{filter:drop-shadow(var(--dfx-shift) 0 0 rgba(255,32,96,.34)) drop-shadow(calc(var(--dfx-shift)*-1) 0 0 rgba(0,214,255,.30));animation:dfx-vhs-wobble 7s steps(1,end) infinite}
.dfx-vhs__scan{background:repeating-linear-gradient(to bottom,rgba(0,0,0,.30) 0,rgba(0,0,0,.30) 1px,transparent 1px,transparent var(--dfx-line));mix-blend-mode:multiply;opacity:var(--dfx-intensity);animation:dfx-vhs-scan 1.1s linear infinite}
.dfx-vhs__head{position:absolute;left:0;right:0;top:0;height:14%;background:linear-gradient(to bottom,transparent,rgba(255,255,255,.30),rgba(255,255,255,.06),transparent);mix-blend-mode:screen;filter:blur(1px);opacity:calc(var(--dfx-intensity)*.85);animation:dfx-vhs-head 6.5s linear infinite}
.dfx-vhs__grain{background-image:url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='120' height='120'%3E%3Cfilter id='g'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='.85' numOctaves='3'/%3E%3C/filter%3E%3Crect width='120' height='120' filter='url(%23g)' opacity='.55'/%3E%3C/svg%3E");mix-blend-mode:overlay;opacity:calc(var(--dfx-intensity)*.5);animation:dfx-vhs-grain 1.2s steps(6,end) infinite}
"#;

/// Worn tape playback over its children: scanlines, chroma bleed, head noise
/// and grain.
///
/// Everything but the colour fringing is an overlay that takes no pointer
/// events, and the fringing is a `drop-shadow` on the content rather than a
/// second copy of it, so the markup underneath is never duplicated and stays
/// selectable. The wobble is a transform, which the browser applies to hit
/// testing too — the page still clicks where it looks.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `VHS`.
#[component]
pub fn Vhs(
    /// Spacing between scanlines, in pixels.
    #[props(default = 3.0)]
    line: f64,
    /// How far the colour channels separate, in pixels.
    #[props(default = 1.6)]
    shift: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    rsx! {
        {dfx_style!("vhs", VHS_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-vhs {class}",
            style: "--dfx-line:{line}px;--dfx-shift:{shift}px;--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-vhs__scan", aria_hidden: "true" }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                span { class: "dfx-vhs__head" }
            }
            span { class: "dfx-surface__layer dfx-vhs__grain", aria_hidden: "true" }
        }
    }
}

const GLITCH_CSS: &str = r#"
@keyframes dfx-glitch-tear{0%,90%,100%{transform:none;filter:none}
91%{transform:translateX(calc(var(--dfx-shift)*-1));filter:drop-shadow(var(--dfx-shift) 0 0 rgba(255,0,72,.6)) drop-shadow(calc(var(--dfx-shift)*-1) 0 0 rgba(0,255,232,.55))}
93%{transform:translateX(var(--dfx-shift));filter:drop-shadow(calc(var(--dfx-shift)*-1) 0 0 rgba(255,0,72,.5))}
95%{transform:none;filter:none}}
@keyframes dfx-glitch-block{0%,90%,100%{opacity:0;transform:none}
91%{opacity:var(--dfx-intensity);transform:translateX(var(--dfx-shift))}
93%{opacity:0;transform:none}
94%{opacity:var(--dfx-intensity);transform:translateX(calc(var(--dfx-shift)*-1.5))}
96%{opacity:0;transform:none}}
.dfx-glitch .dfx-surface__content{animation:dfx-glitch-tear var(--dfx-duration) steps(1,end) infinite}
.dfx-glitch__block{position:absolute;left:0;right:0;top:var(--dfx-top);height:var(--dfx-band);opacity:0;-webkit-backdrop-filter:invert(1) hue-rotate(200deg) saturate(1.6);backdrop-filter:invert(1) hue-rotate(200deg) saturate(1.6);animation:dfx-glitch-block var(--dfx-duration) steps(1,end) var(--dfx-delay) infinite}
@supports not ((-webkit-backdrop-filter:invert(1)) or (backdrop-filter:invert(1))){
.dfx-glitch__block{background:rgba(120,255,240,.35)}
}
"#;

/// Broadcast glitch bursts over its children.
///
/// Idle almost all the time, then a burst: the content jumps sideways with
/// colour fringing while a few bands invert the pixels behind them. The bands
/// are `backdrop-filter`, not copies of the markup, so nothing is duplicated
/// and nothing is hidden — even mid-burst, every word is still on the page.
/// Raise `period` to make it rarer.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Glitch`.
#[component]
pub fn Glitch(
    /// How long one cycle takes, in seconds. The burst is the last tenth of it.
    #[props(default = 6.0)]
    period: f64,
    /// How far the burst displaces things, in pixels.
    #[props(default = 4.0)]
    shift: f64,
    /// How many bands tear during a burst.
    #[props(default = 3)]
    bands: usize,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let bands = bands.min(8);
    rsx! {
        {dfx_style!("glitch", GLITCH_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-glitch {class}",
            style: "--dfx-duration:{period}s;--dfx-shift:{shift}px;--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                for i in 0..bands {
                    {
                        // Spread over the height without overlapping, and stagger
                        // the starts so the bands tear raggedly rather than in
                        // unison. Fixed rather than random, so a rebuild of the
                        // same page glitches the same way.
                        let step = 100.0 / bands as f64;
                        let top = i as f64 * step + step * 0.15;
                        let band = step * 0.4;
                        let delay = i as f64 * period * 0.004;
                        rsx! {
                            span {
                                key: "{i}",
                                class: "dfx-glitch__block",
                                style: "--dfx-top:{top}%;--dfx-band:{band}%;--dfx-delay:{delay}s;",
                            }
                        }
                    }
                }
            }
        }
    }
}

const BLAZE_CSS: &str = r#"
@keyframes dfx-blaze-rise{0%{transform:translate(0,0) scale(1);opacity:0}
14%{opacity:var(--dfx-intensity)}
100%{transform:translate(var(--dfx-drift),-100%) scale(.25);opacity:0}}
@keyframes dfx-blaze-breathe{0%,100%{opacity:calc(var(--dfx-intensity)*.55)}50%{opacity:var(--dfx-intensity)}}
.dfx-blaze__glow{background:linear-gradient(to top,color-mix(in srgb,var(--dfx-color) 60%,transparent),transparent 58%);mix-blend-mode:screen;animation:dfx-blaze-breathe 3.6s ease-in-out infinite}
.dfx-blaze__heat{-webkit-backdrop-filter:blur(var(--dfx-heat));backdrop-filter:blur(var(--dfx-heat));-webkit-mask-image:linear-gradient(to top,#000,transparent 40%);mask-image:linear-gradient(to top,#000,transparent 40%);animation:dfx-blaze-breathe 2.9s ease-in-out infinite}
.dfx-blaze__spark{position:absolute;left:var(--dfx-lane);bottom:0;width:var(--dfx-dot);height:100%;animation:dfx-blaze-rise var(--dfx-duration) linear var(--dfx-delay) infinite}
.dfx-blaze__spark::before{content:"";position:absolute;left:0;bottom:0;width:var(--dfx-dot);height:var(--dfx-dot);border-radius:9999px;background:var(--dfx-color);box-shadow:0 0 8px var(--dfx-color)}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-blaze__heat{background:linear-gradient(to top,color-mix(in srgb,var(--dfx-color) 18%,transparent),transparent 40%)}
}
"#;

/// Embers and heat haze rising over its children.
///
/// A warm glow across the bottom edge, a band of `backdrop-filter` blur
/// standing in for the shimmer above hot air, and sparks drifting up through
/// the content. All of it sits in a layer that takes no pointer events, so it
/// goes over a hero or a footer without touching what is already there.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Blaze`.
#[component]
pub fn Blaze(
    /// Colour of the fire. Any CSS colour.
    #[props(default = "#ff7a18".to_string())]
    color: String,
    /// How many sparks are in flight.
    #[props(default = 14)]
    sparks: usize,
    /// How long a spark takes to cross the box, in seconds.
    #[props(default = 3.2)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let sparks = sparks.min(60);
    let heat = 1.6 * intensity;
    rsx! {
        {dfx_style!("blaze", BLAZE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-blaze {class}",
            style: "--dfx-color:{color};--dfx-intensity:{intensity};--dfx-heat:{heat}px;",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-blaze__heat", aria_hidden: "true" }
            span { class: "dfx-surface__layer dfx-blaze__glow", aria_hidden: "true" }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                for i in 0..sparks {
                    {
                        // Coprime strides scatter the lanes, sizes and drifts
                        // without a random number generator, so the fire looks
                        // irregular and renders identically every time.
                        let lane = ((i * 37 + 11) % 100) as f64;
                        let drift = ((i * 53 % 41) as f64 - 20.0) * intensity;
                        let dot = 2.0 + (i % 3) as f64;
                        let delay = i as f64 * duration / sparks.max(1) as f64;
                        rsx! {
                            span {
                                key: "{i}",
                                class: "dfx-blaze__spark",
                                style: "--dfx-lane:{lane}%;--dfx-drift:{drift}px;--dfx-dot:{dot}px;--dfx-duration:{duration}s;--dfx-delay:{delay}s;",
                            }
                        }
                    }
                }
            }
        }
    }
}

const HALFTONE_CSS: &str = r#"
.dfx-halftone__screen{
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:grayscale(var(--dfx-mono)) contrast(var(--dfx-contrast)) brightness(1.05);
backdrop-filter:grayscale(var(--dfx-mono)) contrast(var(--dfx-contrast)) brightness(1.05);
-webkit-mask-image:radial-gradient(closest-side,#000 42%,transparent 62%);
mask-image:radial-gradient(closest-side,#000 42%,transparent 62%);
-webkit-mask-size:var(--dfx-cell) var(--dfx-cell);
mask-size:var(--dfx-cell) var(--dfx-cell)
}
.dfx-halftone__grid{background-image:radial-gradient(closest-side,color-mix(in srgb,currentColor 45%,transparent) 38%,transparent 56%);background-size:var(--dfx-cell) var(--dfx-cell);mix-blend-mode:multiply;opacity:calc(var(--dfx-intensity)*.4)}
"#;

/// A retro dither screen over its children.
///
/// One dot per cell of `backdrop-filter` — desaturated and pushed in contrast —
/// laid over the untouched content, plus a faint ink grid on top. The result
/// reads as a printed halftone while every glyph underneath stays real text.
/// Nothing animates, so this is the one effect here that costs nothing per
/// frame.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Retro Dither`.
#[component]
pub fn Halftone(
    /// Distance between dot centres, in pixels. Smaller is finer.
    #[props(default = 4.0)]
    cell: f64,
    /// Whether the screened dots drop their colour.
    #[props(default = true)]
    mono: bool,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let mono = if mono { 1 } else { 0 };
    let contrast = 1.0 + 0.5 * intensity;
    rsx! {
        {dfx_style!("halftone", HALFTONE_CSS)}
        div {
            class: "dfx dfx-surface dfx-halftone {class}",
            style: "--dfx-cell:{cell}px;--dfx-mono:{mono};--dfx-contrast:{contrast};--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-halftone__screen", aria_hidden: "true" }
            span { class: "dfx-surface__layer dfx-halftone__grid", aria_hidden: "true" }
        }
    }
}

const BUBBLE_CSS: &str = r#"
.dfx-bubble__ball{
position:absolute;top:0;left:0;width:var(--dfx-dot);height:var(--dfx-dot);
margin:calc(var(--dfx-dot)/-2) 0 0 calc(var(--dfx-dot)/-2);
border-radius:9999px;
opacity:calc(var(--dfx-shown)*var(--dfx-intensity));
transform:translate(var(--dfx-x),var(--dfx-y));
transition:transform var(--dfx-lag) cubic-bezier(.22,1,.36,1),opacity .3s ease;
background:radial-gradient(120% 120% at 30% 26%,rgba(255,255,255,.60),rgba(255,255,255,.05) 44%,transparent 64%),conic-gradient(from 200deg,rgba(255,120,220,.34),rgba(120,220,255,.34),rgba(150,255,190,.34),rgba(255,225,130,.34),rgba(255,120,220,.34));
box-shadow:inset 0 0 0 1px rgba(255,255,255,.45),inset 0 -8px 14px rgba(255,255,255,.18),0 8px 18px rgba(0,0,0,.16);
-webkit-backdrop-filter:blur(var(--dfx-refract)) saturate(1.6) hue-rotate(var(--dfx-hue));
backdrop-filter:blur(var(--dfx-refract)) saturate(1.6) hue-rotate(var(--dfx-hue))
}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-bubble__ball{background-color:rgba(255,255,255,.14)}
}
"#;

/// A string of soap bubbles that trails the pointer and refracts the page.
///
/// Each bubble is `backdrop-filter` under an iridescent film, so the colour
/// inside it is the content behind it — hue-shifted, brightened, and rimmed
/// with a highlight. They lag by increasing amounts, which is what makes the
/// string read as a trail rather than a stack.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Bubble`. Merging
/// the bubbles into metaballs needs a per-pixel alpha threshold across the whole
/// group, which would have to run on an opaque backdrop and so cannot coexist
/// with reading the live page through them; these overlap instead of merging.
#[component]
pub fn Bubble(
    /// Diameter of the leading bubble, in pixels. The rest taper from it.
    #[props(default = 68.0)]
    size: f64,
    /// How many bubbles are in the trail.
    #[props(default = 5)]
    count: usize,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let count = count.clamp(1, 12);
    let shown = if pointer.is_inside() { 1 } else { 0 };
    let position = pointer.position();
    rsx! {
        {dfx_style!("bubble", BUBBLE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-bubble {class}",
            style: "--dfx-intensity:{intensity};--dfx-shown:{shown};{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                // Painted back to front: the last bubble in the trail is the
                // smallest and the slowest, so it has to sit underneath the ones
                // that catch up with the pointer ahead of it.
                for i in (0..count).rev() {
                    {
                        let step = i as f64 / count as f64;
                        let dot = size * (1.0 - 0.62 * step);
                        let lag = 0.18 + 0.26 * i as f64;
                        let hue = 26.0 * i as f64;
                        let refract = 0.6 + 1.4 * intensity;
                        rsx! {
                            span {
                                key: "{i}",
                                class: "dfx-bubble__ball",
                                style: "--dfx-dot:{dot}px;--dfx-lag:{lag}s;--dfx-hue:{hue}deg;--dfx-refract:{refract}px;",
                            }
                        }
                    }
                }
            }
        }
    }
}

const MIST_CSS: &str = r#"
@property --dfx-part{syntax:"<length>";inherits:true;initial-value:0px}
@keyframes dfx-mist-drift{from{background-position:0% 50%,100% 50%,50% 0%}to{background-position:100% 50%,0% 50%,50% 100%}}
.dfx-mist{transition:--dfx-part .55s cubic-bezier(.22,1,.36,1)}
.dfx-mist__bank{
opacity:var(--dfx-intensity);
background-image:radial-gradient(42% 58% at 22% 36%,var(--dfx-tint),transparent 70%),radial-gradient(36% 50% at 70% 28%,var(--dfx-tint),transparent 72%),radial-gradient(50% 46% at 46% 76%,var(--dfx-tint),transparent 68%);
background-size:190% 190%,160% 160%,210% 210%;
-webkit-backdrop-filter:blur(var(--dfx-blur)) saturate(.82) brightness(1.03);
backdrop-filter:blur(var(--dfx-blur)) saturate(.82) brightness(1.03);
-webkit-mask-image:radial-gradient(var(--dfx-part) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 22%,#000 100%);
mask-image:radial-gradient(var(--dfx-part) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 22%,#000 100%);
animation:dfx-mist-drift var(--dfx-duration) linear infinite alternate
}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-mist__bank{background-color:var(--dfx-tint)}
}
"#;

/// Banks of mist drifting over its children, parted by the pointer.
///
/// Where [`Frost`] is a single frozen pane with a crisp hole in it, this is soft
/// and always moving: three gradient banks drifting at different speeds over a
/// `backdrop-filter` blur, with a wide feathered clearing that follows the
/// pointer. The default `tint` is mixed from the `Canvas` system colour, so the
/// mist is pale on a light page and dark on a dark one without being told which
/// it is on.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Clouds`.
#[component]
pub fn Mist(
    /// How heavily the mist blurs what is behind it, in pixels.
    #[props(default = 7.0)]
    blur: f64,
    /// Radius of the clearing the pointer parts, in pixels. `0` never parts.
    #[props(default = 190.0)]
    part: f64,
    /// How long the banks take to drift across, in seconds.
    #[props(default = 24.0)]
    duration: f64,
    /// Colour of the mist. Any CSS colour; use a translucent one.
    #[props(default = "color-mix(in srgb,Canvas 72%,transparent)".to_string())]
    tint: String,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let radius = if pointer.is_inside() {
        part.max(0.0)
    } else {
        0.0
    };
    let position = pointer.position();
    rsx! {
        {dfx_style!("mist", MIST_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-mist {class}",
            style: "--dfx-blur:{blur}px;--dfx-tint:{tint};--dfx-duration:{duration}s;--dfx-intensity:{intensity};--dfx-part:{radius}px;{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-mist__bank", aria_hidden: "true" }
        }
    }
}

const DROPLETS_CSS: &str = r#"
@keyframes dfx-droplets-fall{from{transform:translateY(-14%)}to{transform:translateY(106%)}}
.dfx-droplets__lane{position:absolute;top:0;left:var(--dfx-lane);width:var(--dfx-dot);height:100%;opacity:var(--dfx-intensity);animation:dfx-droplets-fall var(--dfx-duration) linear var(--dfx-delay) infinite}
.dfx-droplets__lane::before{
content:"";position:absolute;top:0;left:0;width:var(--dfx-dot);height:calc(var(--dfx-dot)*1.18);
border-radius:52% 52% 46% 46%/40% 40% 60% 60%;
-webkit-backdrop-filter:blur(var(--dfx-refract)) brightness(1.14) saturate(1.3);
backdrop-filter:blur(var(--dfx-refract)) brightness(1.14) saturate(1.3);
box-shadow:inset 0 -1px 2px rgba(255,255,255,.55),inset 0 2px 3px rgba(0,0,0,.10),0 1px 3px rgba(0,0,0,.18)
}
.dfx-droplets__lane::after{content:"";position:absolute;top:calc(var(--dfx-trail)*-1);left:50%;width:calc(var(--dfx-dot)*.32);height:var(--dfx-trail);transform:translateX(-50%);border-radius:9999px;background:linear-gradient(to top,rgba(255,255,255,.26),transparent)}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-droplets__lane::before{background:rgba(255,255,255,.20)}
}
"#;

/// Rain running down over its children, refracting whatever it crosses.
///
/// Each droplet is a bead of `backdrop-filter` with a rim highlight and the wet
/// streak it left on the way down, falling in its own lane at its own speed.
/// Nothing is drawn on top of the content — a droplet is a window onto it,
/// brightened and pulled slightly out of focus.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Droplets`.
#[component]
pub fn Droplets(
    /// How many droplets are in flight.
    #[props(default = 16)]
    count: usize,
    /// Diameter of the largest droplet, in pixels. The rest vary below it.
    #[props(default = 13.0)]
    size: f64,
    /// How long a droplet takes to cross the box, in seconds.
    #[props(default = 2.6)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let count = count.min(60);
    rsx! {
        {dfx_style!("droplets", DROPLETS_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-droplets {class}",
            style: "--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                for i in 0..count {
                    {
                        // Coprime strides scatter the lanes, sizes and speeds
                        // without a random number generator, so the rain looks
                        // irregular and falls the same way on every render.
                        let lane = ((i * 41 + 7) % 100) as f64;
                        let dot = size * (0.55 + ((i * 29 % 7) as f64) / 13.0);
                        let speed = duration * (0.7 + ((i * 23 % 9) as f64) / 12.0);
                        let delay = -(i as f64) * duration / count.max(1) as f64;
                        let trail = dot * (3.0 + ((i * 17 % 5) as f64));
                        let refract = 0.4 + 1.1 * intensity;
                        rsx! {
                            span {
                                key: "{i}",
                                class: "dfx-droplets__lane",
                                style: "--dfx-lane:{lane}%;--dfx-dot:{dot}px;--dfx-duration:{speed}s;--dfx-delay:{delay}s;--dfx-trail:{trail}px;--dfx-refract:{refract}px;",
                            }
                        }
                    }
                }
            }
        }
    }
}

const TILES_CSS: &str = r#"
@property --dfx-reach{syntax:"<length>";inherits:true;initial-value:0px}
@keyframes dfx-tiles-wave{0%,64%,100%{transform:scale(.80);opacity:.28}32%{transform:scale(1);opacity:1}}
.dfx-tiles{transition:--dfx-reach .45s cubic-bezier(.22,1,.36,1)}
.dfx-tiles__grid{
display:grid;
grid-template-columns:repeat(var(--dfx-cols),1fr);
grid-template-rows:repeat(var(--dfx-rows),1fr);
gap:var(--dfx-gap);
padding:var(--dfx-gap);
opacity:var(--dfx-intensity);
-webkit-mask-image:radial-gradient(var(--dfx-reach) circle at var(--dfx-x) var(--dfx-y),#000 0,#000 34%,transparent 100%);
mask-image:radial-gradient(var(--dfx-reach) circle at var(--dfx-x) var(--dfx-y),#000 0,#000 34%,transparent 100%)
}
.dfx-tiles__tile{
border-radius:var(--dfx-round);
background:linear-gradient(160deg,rgba(255,255,255,.22),rgba(255,255,255,.02) 58%);
box-shadow:inset 0 0 0 1px rgba(255,255,255,.20),0 2px 7px rgba(0,0,0,.16);
-webkit-backdrop-filter:brightness(1.16) saturate(1.3);
backdrop-filter:brightness(1.16) saturate(1.3);
animation:dfx-tiles-wave var(--dfx-duration) ease-in-out var(--dfx-delay) infinite
}
@supports not ((-webkit-backdrop-filter:brightness(1.1)) or (backdrop-filter:brightness(1.1))){
.dfx-tiles__tile{background:rgba(255,255,255,.16)}
}
"#;

/// A grid of lit tiles that ripples in a diagonal wave around the pointer.
///
/// Every tile brightens the content behind it and pulses on a delay set by its
/// place on the diagonal, so the wave crosses the grid rather than pulsing in
/// unison. The whole grid is masked to a circle at the pointer, so the tiles
/// only ever exist where you are looking and the rest of the block is untouched.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Grid`.
#[component]
pub fn Tiles(
    /// How many tiles across.
    #[props(default = 9)]
    columns: usize,
    /// How many tiles down.
    #[props(default = 6)]
    rows: usize,
    /// Radius the pointer lights, in pixels. `0` leaves the grid dark.
    #[props(default = 210.0)]
    reach: f64,
    /// How long one pass of the wave takes, in seconds.
    #[props(default = 2.4)]
    duration: f64,
    /// Corner radius of a tile, in pixels.
    #[props(default = 4.0)]
    radius: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let columns = columns.clamp(1, 24);
    let rows = rows.clamp(1, 24);
    let lit = if pointer.is_inside() {
        reach.max(0.0)
    } else {
        0.0
    };
    let position = pointer.position();
    // The wave has to finish crossing the far corner within one period, or the
    // grid restarts before the last tile has moved.
    let step = duration / (columns + rows) as f64;
    rsx! {
        {dfx_style!("tiles", TILES_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-tiles {class}",
            style: "--dfx-cols:{columns};--dfx-rows:{rows};--dfx-gap:2px;--dfx-round:{radius}px;--dfx-intensity:{intensity};--dfx-reach:{lit}px;{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-tiles__grid", aria_hidden: "true",
                for row in 0..rows {
                    for col in 0..columns {
                        {
                            let delay = (row + col) as f64 * step;
                            rsx! {
                                span {
                                    key: "{row}-{col}",
                                    class: "dfx-tiles__tile",
                                    style: "--dfx-duration:{duration}s;--dfx-delay:{delay}s;",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const HONEYCOMB_CSS: &str = r#"
@keyframes dfx-honeycomb-float{0%,100%{transform:translateY(0)}50%{transform:translateY(calc(var(--dfx-lift)*-1))}}
.dfx-honeycomb__cell{
position:absolute;left:var(--dfx-cx);top:var(--dfx-cy);width:var(--dfx-cw);height:var(--dfx-ch);
clip-path:polygon(50% 0,100% 25%,100% 75%,50% 100%,0 75%,0 25%);
background:linear-gradient(var(--dfx-sheen),rgba(255,255,255,.20),rgba(255,255,255,.02) 46%,rgba(255,255,255,.12));
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:brightness(1.08) saturate(1.28);
backdrop-filter:brightness(1.08) saturate(1.28);
animation:dfx-honeycomb-float var(--dfx-duration) ease-in-out var(--dfx-delay) infinite
}
.dfx-honeycomb__shine{
background:radial-gradient(var(--dfx-glow) circle at var(--dfx-x) var(--dfx-y),rgba(255,255,255,.60),transparent 72%);
mix-blend-mode:screen;
opacity:calc(var(--dfx-shown)*var(--dfx-intensity));
transition:opacity .3s ease
}
@supports not ((-webkit-backdrop-filter:brightness(1.1)) or (backdrop-filter:brightness(1.1))){
.dfx-honeycomb__cell{background-color:rgba(255,255,255,.10)}
}
"#;

/// A honeycomb of floating hex tiles over its children, shining under the
/// pointer.
///
/// The lattice is real hexagons — `clip-path` cells laid on a half-offset grid,
/// each bobbing on its own delay and each brightening the content behind it,
/// so the block reads as sitting on tiles rather than behind a texture. A
/// screen-blended glow tracks the pointer and picks out whichever tiles it
/// crosses.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Hex Float`. The
/// cells are sized as a fraction of the box rather than in pixels, so the
/// lattice keeps its shape at any width; a tall narrow block gets tall narrow
/// hexes.
#[component]
pub fn Honeycomb(
    /// How many hexes across.
    #[props(default = 7)]
    columns: usize,
    /// How many hexes down.
    #[props(default = 5)]
    rows: usize,
    /// How far a hex bobs, in pixels.
    #[props(default = 5.0)]
    lift: f64,
    /// Radius of the glow that follows the pointer, in pixels. `0` never shines.
    #[props(default = 180.0)]
    glow: f64,
    /// How long one bob takes, in seconds.
    #[props(default = 4.0)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let columns = columns.clamp(1, 20);
    let rows = rows.clamp(1, 20);
    let shown = if pointer.is_inside() { 1 } else { 0 };
    let position = pointer.position();

    // Pointy-top hexes tile by overlapping vertically: each row starts three
    // quarters of a cell below the last, and odd rows shift half a cell left, so
    // `rows` of them span `rows * 0.75 + 0.25` cell heights.
    let cell_w = 100.0 / columns as f64;
    let cell_h = 100.0 / (rows as f64 * 0.75 + 0.25);
    // A hair narrower than the pitch, which is what opens the seams between the
    // cells and makes the lattice legible as one.
    let width = cell_w * 0.94;
    let height = cell_h * 0.94;
    rsx! {
        {dfx_style!("honeycomb", HONEYCOMB_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-honeycomb {class}",
            style: "--dfx-cw:{width}%;--dfx-ch:{height}%;--dfx-lift:{lift}px;--dfx-glow:{glow}px;--dfx-intensity:{intensity};--dfx-shown:{shown};{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                for row in 0..rows {
                    // One cell past the edge: the half-cell shift on odd rows
                    // would otherwise leave a notch down the right-hand side.
                    for col in 0..=columns {
                        {
                            let indent = if row % 2 == 1 { cell_w / 2.0 } else { 0.0 };
                            let x = col as f64 * cell_w - indent + (cell_w - width) / 2.0;
                            let y = row as f64 * cell_h * 0.75 + (cell_h - height) / 2.0;
                            let delay = ((row * 3 + col * 5) % 11) as f64 * duration / 11.0;
                            let sheen = 90 + ((row * 7 + col * 13) % 5) * 26;
                            rsx! {
                                span {
                                    key: "{row}-{col}",
                                    class: "dfx-honeycomb__cell",
                                    style: "--dfx-cx:{x}%;--dfx-cy:{y}%;--dfx-duration:{duration}s;--dfx-delay:{delay}s;--dfx-sheen:{sheen}deg;",
                                }
                            }
                        }
                    }
                }
            }
            span { class: "dfx-surface__layer dfx-honeycomb__shine", aria_hidden: "true" }
        }
    }
}

const LASER_CSS: &str = r#"
@property --dfx-cut{syntax:"<percentage>";inherits:true;initial-value:0%}
@keyframes dfx-laser-scan{from{--dfx-cut:0%}to{--dfx-cut:100%}}
@keyframes dfx-laser-fade{0%{opacity:0}6%,88%{opacity:var(--dfx-intensity)}100%{opacity:0}}
.dfx-laser{animation:dfx-laser-scan var(--dfx-duration) cubic-bezier(.45,0,.55,1) var(--dfx-repeat) both}
.dfx-laser .dfx-surface__content{
-webkit-mask-image:linear-gradient(to bottom,#000 var(--dfx-cut),transparent calc(var(--dfx-cut) + 4%));
mask-image:linear-gradient(to bottom,#000 var(--dfx-cut),transparent calc(var(--dfx-cut) + 4%))
}
.dfx-laser__head{position:absolute;left:0;right:0;top:var(--dfx-cut);height:0;opacity:0;animation:dfx-laser-fade var(--dfx-duration) linear var(--dfx-repeat) both}
.dfx-laser__beam{position:absolute;left:0;right:0;top:calc(var(--dfx-thickness)/-2);height:var(--dfx-thickness);background:var(--dfx-color);border-radius:9999px;box-shadow:0 0 var(--dfx-bloom) var(--dfx-color),0 0 calc(var(--dfx-bloom)*2.6) var(--dfx-color)}
.dfx-laser__haze{
position:absolute;left:0;right:0;top:calc(var(--dfx-bloom)/-1);height:calc(var(--dfx-bloom)*2);
background:linear-gradient(to bottom,transparent,color-mix(in srgb,var(--dfx-color) 38%,transparent),transparent);
-webkit-backdrop-filter:blur(2px) brightness(1.25);
backdrop-filter:blur(2px) brightness(1.25)
}
@media (prefers-reduced-motion:reduce){
.dfx-laser{animation:none}
.dfx-laser .dfx-surface__content{-webkit-mask-image:none;mask-image:none}
.dfx-laser__head{display:none}
}
"#;

/// A laser that scans down the block, revealing its children behind it.
///
/// The beam and the reveal are one animation: a registered `--dfx-cut` drives
/// both the mask on the content and the beam's own position, so the line and
/// the edge of the reveal can never drift apart. It plays once by default and
/// leaves the content revealed, which makes it an entrance; set `repeat` and it
/// becomes a scanner that wipes and redraws on a loop.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Laser`, which
/// drives the same idea from scroll position. Unlike every other effect in this
/// module this one hides part of its children while it runs, so under
/// `prefers-reduced-motion` it drops the mask entirely rather than freezing
/// mid-scan with the content still cut away.
#[component]
pub fn Laser(
    /// Colour of the beam. Any CSS colour.
    #[props(default = "#f43f5e".to_string())]
    color: String,
    /// How long one scan takes, in seconds.
    #[props(default = 1.8)]
    duration: f64,
    /// Thickness of the beam itself, in pixels.
    #[props(default = 2.0)]
    thickness: f64,
    /// Whether the scan runs on a loop instead of playing once.
    #[props(default = false)]
    repeat: bool,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let repeat = if repeat { "infinite" } else { "1" };
    let bloom = 6.0 + 14.0 * intensity;
    rsx! {
        {dfx_style!("laser", LASER_CSS)}
        div {
            class: "dfx dfx-surface dfx-laser {class}",
            style: "--dfx-color:{color};--dfx-duration:{duration}s;--dfx-thickness:{thickness}px;--dfx-repeat:{repeat};--dfx-bloom:{bloom}px;--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                span { class: "dfx-laser__head",
                    span { class: "dfx-laser__haze" }
                    span { class: "dfx-laser__beam" }
                }
            }
        }
    }
}

const SHATTER_CSS: &str = r#"
@keyframes dfx-shatter-break{from{opacity:0;transform:scale(.4)}34%{opacity:1}to{opacity:1;transform:scale(1)}}
@keyframes dfx-shatter-flash{from{opacity:.85;transform:translate(var(--dfx-x),var(--dfx-y)) scale(.15)}to{opacity:0;transform:translate(var(--dfx-x),var(--dfx-y)) scale(1.7)}}
.dfx-shatter__glass{transform-origin:var(--dfx-x) var(--dfx-y);animation:dfx-shatter-break var(--dfx-duration) cubic-bezier(.16,1,.3,1) both}
.dfx-shatter__facets{
position:absolute;inset:0;
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:brightness(1.08) saturate(1.25) blur(var(--dfx-refract));
backdrop-filter:brightness(1.08) saturate(1.25) blur(var(--dfx-refract));
-webkit-mask-image:repeating-conic-gradient(from var(--dfx-tilt) at var(--dfx-x) var(--dfx-y),#000 0 var(--dfx-wedge),transparent var(--dfx-wedge) calc(var(--dfx-wedge)*2));
mask-image:repeating-conic-gradient(from var(--dfx-tilt) at var(--dfx-x) var(--dfx-y),#000 0 var(--dfx-wedge),transparent var(--dfx-wedge) calc(var(--dfx-wedge)*2))
}
.dfx-shatter__cracks{
position:absolute;inset:0;
opacity:var(--dfx-intensity);
background:repeating-conic-gradient(from var(--dfx-tilt) at var(--dfx-x) var(--dfx-y),var(--dfx-edge) 0 .3deg,transparent .3deg var(--dfx-wedge)),repeating-radial-gradient(circle at var(--dfx-x) var(--dfx-y),transparent 0 calc(var(--dfx-ring) - 1px),var(--dfx-edge) calc(var(--dfx-ring) - 1px) var(--dfx-ring));
mix-blend-mode:screen
}
.dfx-shatter__flash{
position:absolute;top:0;left:0;width:var(--dfx-ring);height:var(--dfx-ring);
margin:calc(var(--dfx-ring)/-2) 0 0 calc(var(--dfx-ring)/-2);
border-radius:9999px;opacity:0;
background:radial-gradient(closest-side,var(--dfx-edge),transparent 72%);
animation:dfx-shatter-flash var(--dfx-duration) ease-out both
}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-shatter__facets{background:rgba(255,255,255,.10)}
}
"#;

/// Glass that shatters wherever its children are clicked.
///
/// Radial cracks and concentric rings spread from the impact, and the wedges
/// between them alternate through a `backdrop-filter`, so neighbouring facets
/// catch the light differently and the pane reads as broken rather than drawn
/// on. Clicking again moves the impact and re-breaks it. The overlay takes no
/// pointer events, so the links and buttons underneath keep working while the
/// glass is over them.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Shatter`, which
/// lifts each shard into 3D. Displacing a live DOM shard is what CSS cannot do
/// here, so these stay in plane and sell the break through the facets instead.
#[component]
pub fn Shatter(
    /// How many radial shards the pane breaks into.
    #[props(default = 18)]
    shards: usize,
    /// Spacing between the concentric cracks, in pixels.
    #[props(default = 74.0)]
    rings: f64,
    /// How long the break takes to spread, in seconds.
    #[props(default = 0.55)]
    duration: f64,
    /// Colour of the cracks. Any CSS colour; light ones read as lit edges.
    #[props(default = "rgba(255,255,255,.62)".to_string())]
    color: String,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut node = use_signal(|| None::<Rc<MountedData>>);
    // `None` until the first click, which is also what says the impact point is
    // still the centre of the box — a percentage, which the flash's `translate`
    // would resolve against its own size rather than the wrapper's.
    let mut impact = use_signal(|| None::<(usize, f64, f64)>);
    let intensity = intensity.clamp(0.0, 1.0);
    let shards = shards.clamp(3, 90);
    // Each shard needs a lit wedge and an unlit one beside it, so the repeating
    // gradient's period covers two of them.
    let wedge = 180.0 / shards as f64;
    let refract = 0.3 + 0.8 * intensity;
    let hit = impact.cloned();
    let (x, y) = match hit {
        Some((_, x, y)) => (format!("{x}px"), format!("{y}px")),
        None => ("50%".to_string(), "50%".to_string()),
    };
    // Remounting on every click is what replays the break: the animation is
    // `both`-filled, so it would otherwise stay finished forever.
    let key = hit.map(|(id, _, _)| id).unwrap_or(0);
    rsx! {
        {dfx_style!("shatter", SHATTER_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-shatter {class}",
            style: "--dfx-wedge:{wedge}deg;--dfx-tilt:{wedge}deg;--dfx-ring:{rings}px;--dfx-edge:{color};--dfx-duration:{duration}s;--dfx-refract:{refract}px;--dfx-intensity:{intensity};--dfx-x:{x};--dfx-y:{y};",
            onmounted: move |evt| node.set(Some(evt.data())),
            onclick: move |evt: MouseEvent| {
                let handle = node.cloned();
                let point = evt.client_coordinates();
                spawn(async move {
                    let (ox, oy) = viewport_origin(handle).await;
                    let id = impact.cloned().map(|(id, _, _)| id + 1).unwrap_or(1);
                    impact.set(Some((id, point.x - ox, point.y - oy)));
                });
            },
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { key: "{key}", class: "dfx-surface__layer dfx-shatter__glass", aria_hidden: "true",
                span { class: "dfx-shatter__facets" }
                span { class: "dfx-shatter__cracks" }
                if hit.is_some() {
                    span { class: "dfx-shatter__flash" }
                }
            }
        }
    }
}

const STIPPLE_CSS: &str = r#"
@property --dfx-focus{syntax:"<length>";inherits:true;initial-value:0px}
.dfx-stipple{transition:--dfx-focus .4s cubic-bezier(.22,1,.36,1)}
.dfx-stipple__grain{
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:blur(var(--dfx-blur)) contrast(.9) brightness(1.03);
backdrop-filter:blur(var(--dfx-blur)) contrast(.9) brightness(1.03);
-webkit-mask-image:radial-gradient(closest-side,transparent 32%,#000 64%),radial-gradient(var(--dfx-focus) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 36%,#000 100%);
mask-image:radial-gradient(closest-side,transparent 32%,#000 64%),radial-gradient(var(--dfx-focus) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 36%,#000 100%);
-webkit-mask-size:var(--dfx-cell) var(--dfx-cell),100% 100%;
mask-size:var(--dfx-cell) var(--dfx-cell),100% 100%;
-webkit-mask-repeat:repeat,no-repeat;
mask-repeat:repeat,no-repeat;
-webkit-mask-composite:source-in;
mask-composite:intersect
}
.dfx-stipple__specks{
background-image:radial-gradient(closest-side,rgba(255,255,255,.55) 26%,transparent 54%);
background-size:var(--dfx-cell) var(--dfx-cell);
mix-blend-mode:overlay;
opacity:calc(var(--dfx-intensity)*.55);
-webkit-mask-image:radial-gradient(var(--dfx-focus) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 36%,#000 100%);
mask-image:radial-gradient(var(--dfx-focus) circle at var(--dfx-x) var(--dfx-y),transparent 0,transparent 36%,#000 100%)
}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-stipple__grain{background:color-mix(in srgb,Canvas 50%,transparent)}
}
"#;

/// Children rendered as fine grain that resolves crisp around the pointer.
///
/// The blur is masked out at the centre of every cell of a fine grid, so what
/// survives is a field of pin-sharp points on a soft ground — the text is still
/// readable, but it reads as particles until you go looking. A second mask
/// clears both layers wherever the pointer is, which is where the grain merges
/// back into ordinary UI.
///
/// Where [`Halftone`] is a printed screen — static, and the same everywhere —
/// this is a dissolve that follows you.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Particle Reveal`.
#[component]
pub fn Stipple(
    /// Distance between particles, in pixels. Smaller is finer.
    #[props(default = 5.0)]
    cell: f64,
    /// How far the ground between particles is pushed out of focus, in pixels.
    #[props(default = 2.2)]
    blur: f64,
    /// Radius the pointer resolves, in pixels. `0` never resolves.
    #[props(default = 150.0)]
    focus: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let radius = if pointer.is_inside() {
        focus.max(0.0)
    } else {
        0.0
    };
    let position = pointer.position();
    rsx! {
        {dfx_style!("stipple", STIPPLE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-stipple {class}",
            style: "--dfx-cell:{cell}px;--dfx-blur:{blur}px;--dfx-intensity:{intensity};--dfx-focus:{radius}px;{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-stipple__grain", aria_hidden: "true" }
            span { class: "dfx-surface__layer dfx-stipple__specks", aria_hidden: "true" }
        }
    }
}

const LIQUID_CSS: &str = r#"
.dfx-liquid__blob{
position:absolute;top:0;left:0;width:var(--dfx-dot);height:var(--dfx-dot);
margin:calc(var(--dfx-dot)/-2) 0 0 calc(var(--dfx-dot)/-2);
border-radius:9999px;
opacity:calc(var(--dfx-shown)*var(--dfx-intensity)*var(--dfx-weight));
transform:translate(var(--dfx-x),var(--dfx-y));
transition:transform var(--dfx-lag) cubic-bezier(.16,1,.3,1),opacity .45s ease;
-webkit-backdrop-filter:blur(var(--dfx-swirl)) saturate(var(--dfx-saturate)) hue-rotate(var(--dfx-hue)) brightness(1.02);
backdrop-filter:blur(var(--dfx-swirl)) saturate(var(--dfx-saturate)) hue-rotate(var(--dfx-hue)) brightness(1.02);
-webkit-mask-image:radial-gradient(closest-side,#000 16%,rgba(0,0,0,.5) 54%,transparent 100%);
mask-image:radial-gradient(closest-side,#000 16%,rgba(0,0,0,.5) 54%,transparent 100%)
}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-liquid__blob{background:rgba(255,255,255,.10)}
}
"#;

/// A wash of fluid that the pointer drags through its children.
///
/// Soft-edged lobes of `backdrop-filter` follow the pointer at increasing
/// delays, each turning the hue a little further round, and because they carry
/// no rim and fade to nothing at their edges they read as one body of liquid
/// smearing across the content rather than as a row of circles. Where
/// [`Bubble`] is glass — hard edges, a highlight, a shadow — this is the
/// current underneath it.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Liquid`, which
/// runs an actual fluid simulation in WebGL. There is no velocity field here and
/// nothing is being solved: the swirl comes from lag and hue, so it follows the
/// pointer but never curls back on itself.
#[component]
pub fn Liquid(
    /// Diameter of the leading lobe, in pixels.
    #[props(default = 240.0)]
    size: f64,
    /// How many lobes trail the pointer.
    #[props(default = 6)]
    count: usize,
    /// How far each lobe smears what is behind it, in pixels.
    #[props(default = 9.0)]
    swirl: f64,
    /// How far round the hue travels across the trail, in degrees.
    #[props(default = 55.0)]
    hue: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let count = count.clamp(1, 12);
    let shown = if pointer.is_inside() { 1 } else { 0 };
    let position = pointer.position();
    // Every lobe filters the same backdrop, and while the pointer is still they
    // are all stacked on the same spot — so saturation compounds `count` times
    // over. Kept gentle here and thinned per lobe below, or a resting pointer
    // burns a hole in the page.
    let saturate = 1.0 + 0.45 * intensity;
    rsx! {
        {dfx_style!("liquid", LIQUID_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-liquid {class}",
            style: "--dfx-intensity:{intensity};--dfx-shown:{shown};--dfx-saturate:{saturate};{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer", aria_hidden: "true",
                // Back to front: the laggard lobes are the widest and the
                // faintest, and belong under the ones tracking the pointer.
                for i in (0..count).rev() {
                    {
                        let step = i as f64 / count as f64;
                        let dot = size * (1.0 - 0.45 * step);
                        let lag = 0.22 + 0.3 * i as f64;
                        let turn = hue * step;
                        let weight = 1.0 - 0.62 * step;
                        rsx! {
                            span {
                                key: "{i}",
                                class: "dfx-liquid__blob",
                                style: "--dfx-dot:{dot}px;--dfx-lag:{lag}s;--dfx-hue:{turn}deg;--dfx-swirl:{swirl}px;--dfx-weight:{weight};",
                            }
                        }
                    }
                }
            }
        }
    }
}

const DISSOLVE_CSS: &str = r#"
@property --dfx-grain{syntax:"<number>";inherits:true;initial-value:0}
@keyframes dfx-dissolve-settle{from{--dfx-grain:1}to{--dfx-grain:0}}
.dfx-dissolve .dfx-surface__content{
-webkit-mask-image:linear-gradient(to bottom,#000 var(--dfx-line),rgba(0,0,0,calc(1 - var(--dfx-grain)*.82)) calc(var(--dfx-line) + 26%));
mask-image:linear-gradient(to bottom,#000 var(--dfx-line),rgba(0,0,0,calc(1 - var(--dfx-grain)*.82)) calc(var(--dfx-line) + 26%))
}
.dfx-dissolve__sand{
opacity:calc(var(--dfx-grain)*var(--dfx-intensity));
-webkit-backdrop-filter:blur(var(--dfx-blur)) brightness(1.06) saturate(1.1);
backdrop-filter:blur(var(--dfx-blur)) brightness(1.06) saturate(1.1);
-webkit-mask-image:radial-gradient(closest-side,#000 28%,transparent 58%),linear-gradient(to bottom,transparent var(--dfx-line),#000 calc(var(--dfx-line) + 18%));
mask-image:radial-gradient(closest-side,#000 28%,transparent 58%),linear-gradient(to bottom,transparent var(--dfx-line),#000 calc(var(--dfx-line) + 18%));
-webkit-mask-size:var(--dfx-cell) var(--dfx-cell),100% 100%;
mask-size:var(--dfx-cell) var(--dfx-cell),100% 100%;
-webkit-mask-repeat:repeat,no-repeat;
mask-repeat:repeat,no-repeat;
-webkit-mask-composite:source-in;
mask-composite:intersect
}
@supports (animation-timeline:view()){
.dfx-dissolve{animation:dfx-dissolve-settle linear both;animation-timeline:view(block);animation-range:entry 10% cover 62%}
}
"#;

/// Children below a chosen line dissolved into grains that reassemble as the
/// block scrolls in.
///
/// Above `line` the content is untouched; below it, it thins out and a field of
/// sand takes its place — each grain a pinhole of `backdrop-filter` onto what is
/// behind. Scrolling the block into view settles the grains back into solid
/// content, and scrolling it away lets them go again.
///
/// Driven by CSS `animation-timeline: view()`, so there is no observer and no
/// scroll listener. Browsers without scroll-driven animations render it settled
/// — the content whole and the sand gone — rather than stuck dissolved, and so
/// does `prefers-reduced-motion`: the registered `--dfx-grain` starts at zero,
/// which is the readable end of the effect, not the dramatic one.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Particle Scroll`.
#[component]
pub fn Dissolve(
    /// Where the dissolve begins, as a percentage down the block.
    #[props(default = 45.0)]
    line: f64,
    /// Distance between grains, in pixels. Smaller is finer sand.
    #[props(default = 4.0)]
    cell: f64,
    /// How far the grains blur what is behind them, in pixels.
    #[props(default = 1.6)]
    blur: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let line = line.clamp(0.0, 100.0);
    rsx! {
        {dfx_style!("dissolve", DISSOLVE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-dissolve {class}",
            style: "--dfx-line:{line}%;--dfx-cell:{cell}px;--dfx-blur:{blur}px;--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-dissolve__sand", aria_hidden: "true" }
        }
    }
}

const BEND_CSS: &str = r#"
@keyframes dfx-bend-fold{
from{transform:perspective(var(--dfx-depth)) rotateX(var(--dfx-enter))}
50%{transform:perspective(var(--dfx-depth)) rotateX(0deg)}
to{transform:perspective(var(--dfx-depth)) rotateX(var(--dfx-leave))}
}
.dfx-bend__edge{background:linear-gradient(to bottom,rgba(0,0,0,var(--dfx-shade)),transparent var(--dfx-zone),transparent calc(100% - var(--dfx-zone)),rgba(0,0,0,var(--dfx-shade)));opacity:var(--dfx-intensity)}
@supports (animation-timeline:view()){
.dfx-bend .dfx-surface__content{transform-origin:50% 50%;animation:dfx-bend-fold linear both;animation-timeline:view(block)}
}
"#;

/// Children that fold away over a virtual edge as the page scrolls past them.
///
/// Coming up from the bottom the block leans toward you, flattens as it crosses
/// the middle of the viewport, then tips away again over the top edge — the face
/// of a cube turning under the scroll. The shading at the two edges deepens the
/// fold, and because it is all `transform`, hit testing folds with it: the page
/// still clicks where it looks.
///
/// Driven by CSS `animation-timeline: view()`, so there is no scroll listener.
/// Browsers without scroll-driven animations leave the block flat and unfolded.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Bend`, which
/// folds a scroll container's top and bottom bands independently. CSS cannot
/// slice live DOM into bands, so this bends the block as a whole — the same
/// motion at the granularity of a section rather than a scanline.
#[component]
pub fn Bend(
    /// How far the block folds at each edge, in degrees.
    #[props(default = 34.0)]
    angle: f64,
    /// Focal length of the fold, in pixels. Shorter pinches the edge harder.
    #[props(default = 700.0)]
    perspective: f64,
    /// Whether the edges fold away from the reader rather than toward them.
    #[props(default = true)]
    outward: bool,
    /// How deep the shaded band at each edge reaches, as a percentage.
    #[props(default = 22.0)]
    zone: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let angle = angle.clamp(0.0, 89.0);
    // Entering from the bottom the near edge is the top one, so the sign has to
    // flip between the two ends of the timeline or the block would swing one way
    // through the whole scroll instead of folding over an edge at each end.
    let (enter, leave) = if outward {
        (-angle, angle)
    } else {
        (angle, -angle)
    };
    let shade = 0.42 * intensity;
    rsx! {
        {dfx_style!("bend", BEND_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-bend {class}",
            style: "--dfx-depth:{perspective}px;--dfx-enter:{enter}deg;--dfx-leave:{leave}deg;--dfx-zone:{zone}%;--dfx-shade:{shade};--dfx-intensity:{intensity};",
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-bend__edge", aria_hidden: "true" }
        }
    }
}

const SHAPE_CSS: &str = r#"
@keyframes dfx-shape-float{0%,100%{transform:translateY(0) rotate(calc(var(--dfx-tilt)*-1))}50%{transform:translateY(calc(var(--dfx-float)*-1)) rotate(var(--dfx-tilt))}}
.dfx-shape{position:relative;display:inline-block;width:var(--dfx-size);height:var(--dfx-size);vertical-align:middle;animation:dfx-shape-float var(--dfx-duration) ease-in-out infinite}
.dfx-shape__layer{
position:absolute;inset:0;pointer-events:none;
-webkit-mask-image:var(--dfx-shape);mask-image:var(--dfx-shape);
-webkit-mask-size:contain;mask-size:contain;
-webkit-mask-repeat:no-repeat;mask-repeat:no-repeat;
-webkit-mask-position:center;mask-position:center
}
"#;

const GLASS_SHAPE_CSS: &str = r#"
.dfx-glass-shape__body{
-webkit-backdrop-filter:blur(var(--dfx-refract)) saturate(1.7) brightness(1.08);
backdrop-filter:blur(var(--dfx-refract)) saturate(1.7) brightness(1.08);
background:linear-gradient(150deg,rgba(255,255,255,.32),rgba(255,255,255,0) 46%,rgba(255,255,255,.20));
opacity:var(--dfx-intensity)
}
.dfx-glass-shape__fringe{mix-blend-mode:screen;opacity:calc(var(--dfx-intensity)*.5)}
.dfx-glass-shape__fringe--warm{background:rgba(255,64,140,.6);transform:translateX(var(--dfx-split))}
.dfx-glass-shape__fringe--cool{background:rgba(64,200,255,.6);transform:translateX(calc(var(--dfx-split)*-1))}
@supports not ((-webkit-backdrop-filter:blur(1px)) or (backdrop-filter:blur(1px))){
.dfx-glass-shape__body{background-color:rgba(255,255,255,.16)}
}
"#;

/// A silhouette turned into floating glass over whatever is behind it.
///
/// Unlike the rest of this module this one takes no children: `src` is an image
/// or SVG whose alpha becomes the shape, and what fills it is the page behind,
/// refracted. Two offset colour fringes stand in for dispersion, and the whole
/// thing bobs. Drop it over a hero or a gradient and it picks up whatever is
/// there.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Glass Object`,
/// which also accepts a GLB or glTF model and lights it in a studio scene. A
/// mask has no geometry, so this takes the flat inputs only — an SVG, a PNG, an
/// icon — and refracts rather than ray-traces.
#[component]
pub fn GlassShape(
    /// Image or SVG whose alpha cuts the shape. Any URL a CSS `mask-image`
    /// accepts, including a data URI.
    #[props(default = DEFAULT_SHAPE.to_string())]
    src: String,
    /// Width and height of the shape, in pixels.
    #[props(default = 180.0)]
    size: f64,
    /// How far the glass bends what is behind it, in pixels.
    #[props(default = 5.0)]
    refract: f64,
    /// How far the colour fringes separate, in pixels.
    #[props(default = 4.0)]
    split: f64,
    /// How far the shape bobs, in pixels.
    #[props(default = 10.0)]
    float: f64,
    /// How long one bob takes, in seconds.
    #[props(default = 5.0)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let split = split * intensity;
    let tilt = 1.6 * intensity;
    rsx! {
        {dfx_style!("shape", SHAPE_CSS)}
        {dfx_style!("glass-shape", GLASS_SHAPE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-shape dfx-glass-shape {class}",
            style: "--dfx-shape:url('{src}');--dfx-size:{size}px;--dfx-refract:{refract}px;--dfx-split:{split}px;--dfx-float:{float}px;--dfx-duration:{duration}s;--dfx-tilt:{tilt}deg;--dfx-intensity:{intensity};",
            role: "presentation",
            ..attributes,
            span { class: "dfx-shape__layer dfx-glass-shape__fringe dfx-glass-shape__fringe--warm" }
            span { class: "dfx-shape__layer dfx-glass-shape__fringe dfx-glass-shape__fringe--cool" }
            span { class: "dfx-shape__layer dfx-glass-shape__body" }
        }
    }
}

const DITHER_SHAPE_CSS: &str = r#"
.dfx-dither-shape__screen{
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:grayscale(var(--dfx-mono)) contrast(var(--dfx-contrast)) brightness(1.08);
backdrop-filter:grayscale(var(--dfx-mono)) contrast(var(--dfx-contrast)) brightness(1.08);
-webkit-mask-image:var(--dfx-shape),radial-gradient(closest-side,#000 40%,transparent 62%);
mask-image:var(--dfx-shape),radial-gradient(closest-side,#000 40%,transparent 62%);
-webkit-mask-size:contain,var(--dfx-cell) var(--dfx-cell);
mask-size:contain,var(--dfx-cell) var(--dfx-cell);
-webkit-mask-repeat:no-repeat,repeat;
mask-repeat:no-repeat,repeat;
-webkit-mask-position:center,0 0;
mask-position:center,0 0;
-webkit-mask-composite:source-in;
mask-composite:intersect
}
.dfx-dither-shape__ink{background:var(--dfx-ink);opacity:calc(var(--dfx-intensity)*.09)}
"#;

/// A silhouette screened through a one-bit dither, over whatever is behind it.
///
/// The shape is cut from `src`, and what fills it is the page behind pushed to
/// grayscale and hard contrast, then punched through a dot grid — the printed
/// look of [`Halftone`] confined to an outline. Takes no children; it is an
/// object to place, not a wrapper.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s
/// `Dithered Object`, which dithers a lit 3D model. This dithers the live page
/// seen through the shape instead, which is the version that has something to
/// read from.
#[component]
pub fn DitherShape(
    /// Image or SVG whose alpha cuts the shape. Any URL a CSS `mask-image`
    /// accepts, including a data URI.
    #[props(default = DEFAULT_SHAPE.to_string())]
    src: String,
    /// Width and height of the shape, in pixels.
    #[props(default = 180.0)]
    size: f64,
    /// Distance between dot centres, in pixels. Smaller is finer.
    #[props(default = 4.0)]
    cell: f64,
    /// Whether the screened dots drop their colour.
    #[props(default = true)]
    mono: bool,
    /// Colour of the ink laid under the screen. Any CSS colour.
    #[props(default = "currentColor".to_string())]
    ink: String,
    /// How far the shape bobs, in pixels.
    #[props(default = 8.0)]
    float: f64,
    /// How long one bob takes, in seconds.
    #[props(default = 6.0)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    let mono = if mono { 1 } else { 0 };
    let contrast = 1.0 + 1.4 * intensity;
    let tilt = 1.2 * intensity;
    rsx! {
        {dfx_style!("shape", SHAPE_CSS)}
        {dfx_style!("dither-shape", DITHER_SHAPE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-shape dfx-dither-shape {class}",
            style: "--dfx-shape:url('{src}');--dfx-size:{size}px;--dfx-cell:{cell}px;--dfx-mono:{mono};--dfx-contrast:{contrast};--dfx-ink:{ink};--dfx-float:{float}px;--dfx-duration:{duration}s;--dfx-tilt:{tilt}deg;--dfx-intensity:{intensity};",
            role: "presentation",
            ..attributes,
            span { class: "dfx-shape__layer dfx-dither-shape__ink" }
            span { class: "dfx-shape__layer dfx-dither-shape__screen" }
        }
    }
}

const PARTICLE_SHAPE_CSS: &str = r#"
.dfx-particle-shape__grains{
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:brightness(1.18) saturate(1.45);
backdrop-filter:brightness(1.18) saturate(1.45);
-webkit-mask-image:var(--dfx-shape),radial-gradient(closest-side,#000 32%,transparent 58%);
mask-image:var(--dfx-shape),radial-gradient(closest-side,#000 32%,transparent 58%);
-webkit-mask-size:contain,var(--dfx-cell) var(--dfx-cell);
mask-size:contain,var(--dfx-cell) var(--dfx-cell);
-webkit-mask-repeat:no-repeat,repeat;
mask-repeat:no-repeat,repeat;
-webkit-mask-position:center,0 0;
mask-position:center,0 0;
-webkit-mask-composite:source-in;
mask-composite:intersect;
transition:transform .55s cubic-bezier(.22,1,.36,1),opacity .55s ease
}
.dfx-particle-shape:hover .dfx-particle-shape__grains,.dfx-particle-shape:focus-within .dfx-particle-shape__grains{transform:scale(var(--dfx-scatter));opacity:calc(var(--dfx-intensity)*.6)}
.dfx-particle-shape:hover .dfx-particle-shape__grains--in,.dfx-particle-shape:focus-within .dfx-particle-shape__grains--in{transform:scale(calc(2 - var(--dfx-scatter)))}
"#;

/// A silhouette rebuilt as particles that scatter under the pointer and spring
/// back.
///
/// The shape is cut from `src` and punched through a fine grid, so what is left
/// is a cloud of points each showing the page behind it. Two clouds, in fact:
/// on hover one flies outward and the other inward, which is what pulls the
/// outline apart. Both ease back into register when the pointer leaves. Takes no
/// children.
///
/// Reacts to `:focus-within` as well as `:hover`, so it scatters for keyboard
/// users too.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s
/// `Particle Object`, where each particle is a real point pushed by a real force
/// field. These two clouds move as wholes; the scatter is in the outline, not
/// per grain.
#[component]
pub fn ParticleShape(
    /// Image or SVG whose alpha cuts the shape. Any URL a CSS `mask-image`
    /// accepts, including a data URI.
    #[props(default = DEFAULT_SHAPE.to_string())]
    src: String,
    /// Width and height of the shape, in pixels.
    #[props(default = 180.0)]
    size: f64,
    /// Distance between particles, in pixels. Smaller is finer.
    #[props(default = 5.0)]
    cell: f64,
    /// How far the cloud flies apart on hover, as a scale factor above `1`.
    #[props(default = 1.25)]
    scatter: f64,
    /// How far the shape bobs, in pixels.
    #[props(default = 8.0)]
    float: f64,
    /// How long one bob takes, in seconds.
    #[props(default = 5.5)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let intensity = intensity.clamp(0.0, 1.0);
    // Below 1 the two clouds would swap which way they fly, which reads as the
    // shape imploding rather than scattering.
    let scatter = 1.0 + (scatter - 1.0).max(0.0) * intensity;
    let tilt = 1.2 * intensity;
    rsx! {
        {dfx_style!("shape", SHAPE_CSS)}
        {dfx_style!("particle-shape", PARTICLE_SHAPE_CSS)}
        div {
            class: "dfx dfx-decorative dfx-shape dfx-particle-shape {class}",
            style: "--dfx-shape:url('{src}');--dfx-size:{size}px;--dfx-cell:{cell}px;--dfx-scatter:{scatter};--dfx-float:{float}px;--dfx-duration:{duration}s;--dfx-tilt:{tilt}deg;--dfx-intensity:{intensity};",
            role: "presentation",
            ..attributes,
            span { class: "dfx-shape__layer dfx-particle-shape__grains dfx-particle-shape__grains--in" }
            span { class: "dfx-shape__layer dfx-particle-shape__grains" }
        }
    }
}

const ASCII_CSS: &str = r#"
@property --dfx-lens{syntax:"<length>";inherits:true;initial-value:0px}
.dfx-ascii{transition:--dfx-lens .35s cubic-bezier(.22,1,.36,1)}
.dfx-ascii__ink{
opacity:var(--dfx-intensity);
-webkit-backdrop-filter:grayscale(var(--dfx-mono)) contrast(var(--dfx-contrast)) brightness(1.18);
backdrop-filter:grayscale(var(--dfx-mono)) contrast(var(--dfx-contrast)) brightness(1.18);
-webkit-mask-image:radial-gradient(var(--dfx-lens) circle at var(--dfx-x) var(--dfx-y),#000 0,#000 62%,transparent 100%);
mask-image:radial-gradient(var(--dfx-lens) circle at var(--dfx-x) var(--dfx-y),#000 0,#000 62%,transparent 100%)
}
.dfx-ascii__lens{
-webkit-mask-image:radial-gradient(var(--dfx-lens) circle at var(--dfx-x) var(--dfx-y),#000 0,#000 62%,transparent 100%);
mask-image:radial-gradient(var(--dfx-lens) circle at var(--dfx-x) var(--dfx-y),#000 0,#000 62%,transparent 100%)
}
.dfx-ascii__screen{
position:absolute;inset:0;
background:var(--dfx-ground);
opacity:var(--dfx-intensity);
-webkit-mask-image:url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='48' height='48'%3E%3Cdefs%3E%3Cmask id='a'%3E%3Crect width='48' height='48' fill='%23fff'/%3E%3Ctext x='1' y='14' font-family='monospace' font-size='15' fill='%23000'%3E@WM%3C/text%3E%3Ctext x='1' y='30' font-family='monospace' font-size='15' fill='%23000'%3E*%2B:%3C/text%3E%3Ctext x='1' y='46' font-family='monospace' font-size='15' fill='%23000'%3E.oX%3C/text%3E%3C/mask%3E%3C/defs%3E%3Crect width='48' height='48' fill='%23000' mask='url(%23a)'/%3E%3C/svg%3E");
mask-image:url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='48' height='48'%3E%3Cdefs%3E%3Cmask id='a'%3E%3Crect width='48' height='48' fill='%23fff'/%3E%3Ctext x='1' y='14' font-family='monospace' font-size='15' fill='%23000'%3E@WM%3C/text%3E%3Ctext x='1' y='30' font-family='monospace' font-size='15' fill='%23000'%3E*%2B:%3C/text%3E%3Ctext x='1' y='46' font-family='monospace' font-size='15' fill='%23000'%3E.oX%3C/text%3E%3C/mask%3E%3C/defs%3E%3Crect width='48' height='48' fill='%23000' mask='url(%23a)'/%3E%3C/svg%3E");
-webkit-mask-size:var(--dfx-cell) var(--dfx-cell);
mask-size:var(--dfx-cell) var(--dfx-cell);
-webkit-mask-repeat:repeat;
mask-repeat:repeat
}
"#;

/// A lens that follows the pointer and leaves the page showing only through
/// character-shaped holes.
///
/// Inside the lens a sheet of `ground` covers the content, punched through with
/// a tiled field of monospace glyphs, so what you read there is the live page
/// seen through `@`, `W`, `M`, `*`, `+`, `:`, `.`, `o` and `X` — contrast pushed
/// and colour dropped underneath, the way a terminal would render it. Outside
/// the lens nothing is touched.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Asciify`. Real
/// ASCII art picks the character that matches each cell's brightness, which
/// needs to read the rendered pixels back — CSS can do no such thing. The glyph
/// field here is fixed, and the image comes from which parts of the page fall
/// behind which characters rather than from choosing them. Same alphabet, same
/// lens, no sampling: closer to [`Halftone`]'s fixed dot grid than to a real
/// converter.
#[component]
pub fn Ascii(
    /// Size of one glyph tile, in pixels. Smaller is denser text.
    #[props(default = 15.0)]
    cell: f64,
    /// Radius of the lens, in pixels. `0` never opens.
    #[props(default = 170.0)]
    lens: f64,
    /// Colour of the sheet the glyphs are cut out of. Any CSS colour.
    #[props(default = "rgba(6,10,20,.92)".to_string())]
    ground: String,
    /// Whether the page inside the lens drops its colour.
    #[props(default = true)]
    mono: bool,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    let mono = if mono { 1 } else { 0 };
    let contrast = 1.0 + 0.9 * intensity;
    let radius = if pointer.is_inside() {
        lens.max(0.0)
    } else {
        0.0
    };
    let position = pointer.position();
    rsx! {
        {dfx_style!("ascii", ASCII_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-ascii {class}",
            style: "--dfx-cell:{cell}px;--dfx-ground:{ground};--dfx-mono:{mono};--dfx-contrast:{contrast};--dfx-intensity:{intensity};--dfx-lens:{radius}px;{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            // The ink goes under the screen, not over it: a `backdrop-filter`
            // samples whatever is painted beneath it, so filtering above the
            // ground sheet would only ever grade the ground.
            span { class: "dfx-surface__layer dfx-ascii__ink", aria_hidden: "true" }
            // The lens clips the glyph sheet by nesting rather than by
            // `mask-composite`: one mask per element always composites, and the
            // sheet paints a flat colour, so there is no `backdrop-filter` here
            // for the extra group to cut off from the page behind it.
            span { class: "dfx-surface__layer dfx-ascii__lens", aria_hidden: "true",
                span { class: "dfx-ascii__screen" }
            }
        }
    }
}

const CLOTH_CSS: &str = r#"
@keyframes dfx-cloth-sway{0%,100%{transform:skewY(calc(var(--dfx-sway)*-1)) translateY(calc(var(--dfx-lift)*-1))}50%{transform:skewY(var(--dfx-sway)) translateY(var(--dfx-lift))}}
@keyframes dfx-cloth-drift{from{background-position:0 0}to{background-position:var(--dfx-fold) 0}}
.dfx-cloth .dfx-surface__content{animation:dfx-cloth-sway var(--dfx-duration) ease-in-out infinite}
.dfx-cloth__weave{
background-image:repeating-linear-gradient(90deg,rgba(0,0,0,.30) 0 1px,transparent 1px var(--dfx-thread)),repeating-linear-gradient(0deg,rgba(255,255,255,.30) 0 1px,transparent 1px var(--dfx-thread));
opacity:calc(var(--dfx-intensity)*.7);
mix-blend-mode:overlay
}
.dfx-cloth__folds{
background-image:repeating-linear-gradient(100deg,rgba(0,0,0,.45) 0,transparent 15%,rgba(255,255,255,.42) 30%,transparent 46%,rgba(0,0,0,.45) 60%);
background-size:var(--dfx-fold) 100%;
opacity:calc(var(--dfx-intensity)*.85);
mix-blend-mode:overlay;
animation:dfx-cloth-drift var(--dfx-duration) linear infinite
}
.dfx-cloth__swell{
background:radial-gradient(var(--dfx-reach) circle at var(--dfx-x) var(--dfx-y),rgba(255,255,255,.40),transparent 70%);
mix-blend-mode:screen;
opacity:calc(var(--dfx-shown)*var(--dfx-intensity));
transition:opacity .45s ease
}
"#;

/// Its children hung on fabric that breathes, with folds crossing it and a
/// swell wherever the pointer rests.
///
/// A weave of hairline threads over the content, bands of light and shade
/// travelling across it, and a gentle skew and lift on the content itself, so
/// the whole block moves as one sheet rather than sitting behind a texture.
/// The skew is a `transform`, which the browser applies to hit testing too, so
/// the page still clicks where it looks.
///
/// After [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s `Cloth`, which
/// hangs the page on a simulated mesh and propagates a wave from every cursor
/// stroke. There is no mesh here and nothing is being solved: the fabric is
/// shading, the wind is one long keyframe, and the pointer lights the cloth
/// rather than pushing it. What it borrows is the read, not the physics.
#[component]
pub fn Cloth(
    /// Spacing of the threads in the weave, in pixels.
    #[props(default = 4.0)]
    thread: f64,
    /// Distance between folds, in pixels.
    #[props(default = 190.0)]
    fold: f64,
    /// How far the sheet skews as it breathes, in degrees.
    #[props(default = 0.35)]
    sway: f64,
    /// Radius of the swell under the pointer, in pixels. `0` never swells.
    #[props(default = 180.0)]
    reach: f64,
    /// How long one breath takes, in seconds.
    #[props(default = 7.0)]
    duration: f64,
    /// How present the effect is, from `0` to `1`.
    #[props(default = 1.0)]
    intensity: f64,
    /// Extra classes for the wrapper element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut pointer = use_local_pointer();
    let intensity = intensity.clamp(0.0, 1.0);
    // Enough to read as movement, never enough to make the text lean.
    let sway = sway.clamp(0.0, 3.0) * intensity;
    let lift = 3.0 * intensity;
    let shown = if pointer.is_inside() { 1 } else { 0 };
    let position = pointer.position();
    rsx! {
        {dfx_style!("cloth", CLOTH_CSS)}
        div {
            class: "dfx dfx-decorative dfx-surface dfx-cloth {class}",
            style: "--dfx-thread:{thread}px;--dfx-fold:{fold}px;--dfx-sway:{sway}deg;--dfx-lift:{lift}px;--dfx-reach:{reach}px;--dfx-duration:{duration}s;--dfx-intensity:{intensity};--dfx-shown:{shown};{position}",
            onmounted: move |evt| pointer.mounted(evt.data()),
            onmouseenter: move |_| pointer.enter(),
            onmousemove: move |evt| pointer.moved(&evt),
            onmouseleave: move |_| pointer.leave(),
            ..attributes,
            div { class: "dfx-surface__content", {children} }
            span { class: "dfx-surface__layer dfx-cloth__weave", aria_hidden: "true" }
            span { class: "dfx-surface__layer dfx-cloth__folds", aria_hidden: "true" }
            span { class: "dfx-surface__layer dfx-cloth__swell", aria_hidden: "true" }
        }
    }
}
