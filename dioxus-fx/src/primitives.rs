//! Enter and exit animations for headless component libraries.
//!
//! [`dioxus-primitives`](https://github.com/DioxusLabs/dioxus-components) — the
//! crate behind <https://dioxuslabs.com/components> — ships unstyled components
//! that describe their state with Radix-style data attributes: a dialog carries
//! `data-state="open"` or `data-state="closed"`, an accordion panel carries
//! `data-open="true"` or `data-open="false"`, and positioned content carries
//! `data-side` and `data-align`. This module is a stylesheet keyed on exactly
//! those attributes, so animating one is adding a class:
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! # macro_rules! stub { () => {} }
//! use dioxus_fx::primitives::*;
//!
//! fn Sheet() -> Element {
//!     rsx! {
//!         PrimitivesStyle {}
//!         // DialogRoot { class: "dx-dialog {DFX_ZOOM}", .. }
//!         // AccordionContent { class: "{DFX_COLLAPSE}", .. }
//!     }
//! }
//! ```
//!
//! There is no dependency on `dioxus-primitives` here, and nothing to wire up
//! beyond the class: the rules match the attributes, whichever library writes
//! them.
//!
//! # Exit animations
//!
//! `dioxus-primitives` keeps closing content in the DOM until every animation
//! **on the element carrying the state attribute** has finished, so the `closed`
//! half of each pair actually plays. Three consequences shape these rules:
//!
//! - Always put a class on the element that carries the state — `DialogRoot`,
//!   `PopoverContent`, `TooltipContent`, `AccordionContent`. That animation is
//!   what holds the subtree in the DOM.
//! - A dialog marks only its root, so animating the panel inside it works
//!   through a descendant match: a class on `DialogContent` animates when an
//!   ancestor opens or closes. Keep the inner duration at or under the root's,
//!   or the inner animation is cut off when the root unmounts.
//! - Under `prefers-reduced-motion` the durations collapse to `0.01ms` rather
//!   than to `animation: none`, so closing content still ends up hidden and
//!   still unmounts promptly.
//!
//! # Composing with the library's own CSS
//!
//! Positioned content is usually centred with `transform: translateX(-50%)` or
//! similar. These keyframes animate the independent `translate`, `scale` and
//! `opacity` properties instead of the `transform` shorthand, so they compose
//! with that positioning rather than overwriting it.
//!
//! Timing is per-element, through custom properties:
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! # use dioxus_fx::primitives::*;
//! # fn f() -> Element {
//! rsx! {
//!     div { class: "{DFX_SLIDE}", style: "--dfx-enter:.24s;--dfx-shift:12px;" }
//! }
//! # }
//! ```

use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[STATE_CSS];

/// Fade in and out. The safe default, and the one to use on dialog backdrops.
pub const DFX_FADE: &str = "dfx-state-fade";

/// Fade while scaling up from `--dfx-scale-from`. Suits dialogs and popovers.
pub const DFX_ZOOM: &str = "dfx-state-zoom";

/// Fade while sliding out of the trigger, in the direction `data-side` implies.
///
/// Without a `data-side` the content rises, which is what an unpositioned
/// element wants anyway.
pub const DFX_SLIDE: &str = "dfx-state-slide";

/// [`DFX_ZOOM`] anchored to the trigger edge, so menus grow out of their button.
///
/// The origin follows `data-side` and `data-align`; with neither it grows down
/// from the top.
pub const DFX_MENU: &str = "dfx-state-menu";

/// Collapse and expand height, for accordion and collapsible content.
///
/// Animates `grid-template-rows` between `0fr` and `1fr`, so the panel measures
/// itself and no height has to be known up front. The class sets `display:
/// grid` on the panel; its children are clipped while it moves.
///
/// Padding and margins on the direct child survive the collapse — the row goes
/// to zero, that box does not. Wrap the content and space it inside:
///
/// ```rust, no_run
/// # use dioxus::prelude::*;
/// # use dioxus_fx::primitives::DFX_COLLAPSE;
/// # fn f() -> Element {
/// rsx! {
///     div { class: "{DFX_COLLAPSE}", "data-open": "true",
///         div {
///             p { style: "padding: 8px 0", "Spacing goes here, not on the wrapper." }
///         }
///     }
/// }
/// # }
/// ```
pub const DFX_COLLAPSE: &str = "dfx-state-collapse";

// Each effect matches four ways: the state attribute on the element itself, in
// either spelling, and the same two on an ancestor — a dialog only marks its
// root, so animating the panel inside it needs the descendant form. Open rules
// come before closed rules throughout, so that an element carrying its own
// `closed` inside an `open` ancestor still closes: the selectors tie on
// specificity, and the later rule wins.
const STATE_CSS: &str = r#"
.dfx-state-fade,.dfx-state-zoom,.dfx-state-slide,.dfx-state-menu{--dfx-enter:.18s;--dfx-exit:.14s;--dfx-enter-ease:cubic-bezier(.16,1,.3,1);--dfx-exit-ease:cubic-bezier(.4,0,1,1);--dfx-shift:6px;--dfx-scale-from:.96}
.dfx-state-fade[data-state=open],.dfx-state-fade[data-open=true],[data-state=open] .dfx-state-fade,[data-open=true] .dfx-state-fade{animation:dfx-state-fade-in var(--dfx-enter) var(--dfx-enter-ease) both}
.dfx-state-fade[data-state=closed],.dfx-state-fade[data-open=false],[data-state=closed] .dfx-state-fade,[data-open=false] .dfx-state-fade{animation:dfx-state-fade-out var(--dfx-exit) var(--dfx-exit-ease) both;pointer-events:none}
@keyframes dfx-state-fade-in{from{opacity:0}to{opacity:1}}
@keyframes dfx-state-fade-out{from{opacity:1}to{opacity:0}}
.dfx-state-zoom[data-state=open],.dfx-state-zoom[data-open=true],[data-state=open] .dfx-state-zoom,[data-open=true] .dfx-state-zoom,.dfx-state-menu[data-state=open],.dfx-state-menu[data-open=true],[data-state=open] .dfx-state-menu,[data-open=true] .dfx-state-menu{animation:dfx-state-zoom-in var(--dfx-enter) var(--dfx-enter-ease) both}
.dfx-state-zoom[data-state=closed],.dfx-state-zoom[data-open=false],[data-state=closed] .dfx-state-zoom,[data-open=false] .dfx-state-zoom,.dfx-state-menu[data-state=closed],.dfx-state-menu[data-open=false],[data-state=closed] .dfx-state-menu,[data-open=false] .dfx-state-menu{animation:dfx-state-zoom-out var(--dfx-exit) var(--dfx-exit-ease) both;pointer-events:none}
@keyframes dfx-state-zoom-in{from{opacity:0;scale:var(--dfx-scale-from)}to{opacity:1;scale:1}}
@keyframes dfx-state-zoom-out{from{opacity:1;scale:1}to{opacity:0;scale:var(--dfx-scale-from)}}
.dfx-state-slide{--dfx-slide:0 var(--dfx-shift)}
.dfx-state-slide[data-side=bottom]{--dfx-slide:0 calc(var(--dfx-shift) * -1)}
.dfx-state-slide[data-side=left]{--dfx-slide:var(--dfx-shift) 0}
.dfx-state-slide[data-side=right]{--dfx-slide:calc(var(--dfx-shift) * -1) 0}
.dfx-state-slide[data-state=open],.dfx-state-slide[data-open=true],[data-state=open] .dfx-state-slide,[data-open=true] .dfx-state-slide{animation:dfx-state-slide-in var(--dfx-enter) var(--dfx-enter-ease) both}
.dfx-state-slide[data-state=closed],.dfx-state-slide[data-open=false],[data-state=closed] .dfx-state-slide,[data-open=false] .dfx-state-slide{animation:dfx-state-slide-out var(--dfx-exit) var(--dfx-exit-ease) both;pointer-events:none}
@keyframes dfx-state-slide-in{from{opacity:0;translate:var(--dfx-slide)}to{opacity:1;translate:none}}
@keyframes dfx-state-slide-out{from{opacity:1;translate:none}to{opacity:0;translate:var(--dfx-slide)}}
.dfx-state-menu{--dfx-origin-x:center;--dfx-origin-y:top;transform-origin:var(--dfx-origin-x) var(--dfx-origin-y)}
.dfx-state-menu[data-align=start]{--dfx-origin-x:left}
.dfx-state-menu[data-align=end]{--dfx-origin-x:right}
.dfx-state-menu[data-side=top]{--dfx-origin-y:bottom}
.dfx-state-menu[data-side=left]{--dfx-origin-x:right;--dfx-origin-y:center}
.dfx-state-menu[data-side=right]{--dfx-origin-x:left;--dfx-origin-y:center}
.dfx-state-collapse{--dfx-enter:.22s;--dfx-exit:.18s;--dfx-enter-ease:cubic-bezier(.16,1,.3,1);--dfx-exit-ease:cubic-bezier(.4,0,1,1);display:grid;grid-template-rows:1fr}
.dfx-state-collapse>*{min-height:0;overflow:hidden}
.dfx-state-collapse[data-state=open],.dfx-state-collapse[data-open=true]{animation:dfx-state-collapse-open var(--dfx-enter) var(--dfx-enter-ease) both}
.dfx-state-collapse[data-state=closed],.dfx-state-collapse[data-open=false]{animation:dfx-state-collapse-close var(--dfx-exit) var(--dfx-exit-ease) both}
@keyframes dfx-state-collapse-open{from{grid-template-rows:0fr;opacity:0}to{grid-template-rows:1fr;opacity:1}}
@keyframes dfx-state-collapse-close{from{grid-template-rows:1fr;opacity:1}to{grid-template-rows:0fr;opacity:0}}
@media (prefers-reduced-motion:reduce){
.dfx-state-fade,.dfx-state-zoom,.dfx-state-slide,.dfx-state-menu,.dfx-state-collapse{--dfx-enter:.01ms;--dfx-exit:.01ms}
}
"#;

/// Mounts the state-attribute stylesheet.
///
/// Render this once, anywhere above the components you have added an
/// `DFX_`-prefixed class to. The other modules in this crate inject their CSS
/// from the components themselves; these classes go on components this crate
/// does not own, so there is nothing to hang that on — hence the explicit
/// mount. It is deduplicated by key, so rendering it more than once is free,
/// and [`FxStyle`](crate::FxStyle) already includes these rules.
///
/// ```rust, no_run
/// # use dioxus::prelude::*;
/// use dioxus_fx::primitives::{PrimitivesStyle, DFX_ZOOM};
///
/// fn App() -> Element {
///     rsx! {
///         PrimitivesStyle {}
///         div { class: "{DFX_ZOOM}", "data-state": "open", "Hello" }
///     }
/// }
/// ```
#[component]
pub fn PrimitivesStyle() -> Element {
    rsx! {
        {dfx_style!("state", STATE_CSS)}
    }
}
