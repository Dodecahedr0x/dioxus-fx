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
//! [`Frost`], [`Lens`], [`Glitch`], [`Blaze`] and [`Halftone`] read the content
//! beneath them with `backdrop-filter`. Where that is unsupported each falls
//! back to a plain translucent layer, which is dimmer but never blank.
//! [`Peel`], [`Ripple`] and [`Vhs`] need nothing beyond transforms, masks and
//! keyframes.

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
];

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
