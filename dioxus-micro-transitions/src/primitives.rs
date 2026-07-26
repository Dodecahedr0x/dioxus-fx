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
//! use dioxus_micro_transitions::primitives::*;
//!
//! fn Sheet() -> Element {
//!     rsx! {
//!         PrimitivesStyle {}
//!         // DialogRoot { class: "dx-dialog {AMT_ZOOM}", .. }
//!         // AccordionContent { class: "{AMT_COLLAPSE}", .. }
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
//! # use dioxus_micro_transitions::primitives::*;
//! # fn f() -> Element {
//! rsx! {
//!     div { class: "{AMT_SLIDE}", style: "--amt-enter:.24s;--amt-shift:12px;" }
//! }
//! # }
//! ```

use crate::style::amt_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[STATE_CSS];

/// Fade in and out. The safe default, and the one to use on dialog backdrops.
pub const AMT_FADE: &str = "amt-state-fade";

/// Fade while scaling up from `--amt-scale-from`. Suits dialogs and popovers.
pub const AMT_ZOOM: &str = "amt-state-zoom";

/// Fade while sliding out of the trigger, in the direction `data-side` implies.
///
/// Without a `data-side` the content rises, which is what an unpositioned
/// element wants anyway.
pub const AMT_SLIDE: &str = "amt-state-slide";

/// [`AMT_ZOOM`] anchored to the trigger edge, so menus grow out of their button.
///
/// The origin follows `data-side` and `data-align`; with neither it grows down
/// from the top.
pub const AMT_MENU: &str = "amt-state-menu";

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
/// # use dioxus_micro_transitions::primitives::AMT_COLLAPSE;
/// # fn f() -> Element {
/// rsx! {
///     div { class: "{AMT_COLLAPSE}", "data-open": "true",
///         div {
///             p { style: "padding: 8px 0", "Spacing goes here, not on the wrapper." }
///         }
///     }
/// }
/// # }
/// ```
pub const AMT_COLLAPSE: &str = "amt-state-collapse";

// Each effect matches four ways: the state attribute on the element itself, in
// either spelling, and the same two on an ancestor — a dialog only marks its
// root, so animating the panel inside it needs the descendant form. Open rules
// come before closed rules throughout, so that an element carrying its own
// `closed` inside an `open` ancestor still closes: the selectors tie on
// specificity, and the later rule wins.
const STATE_CSS: &str = r#"
.amt-state-fade,.amt-state-zoom,.amt-state-slide,.amt-state-menu{--amt-enter:.18s;--amt-exit:.14s;--amt-enter-ease:cubic-bezier(.16,1,.3,1);--amt-exit-ease:cubic-bezier(.4,0,1,1);--amt-shift:6px;--amt-scale-from:.96}
.amt-state-fade[data-state=open],.amt-state-fade[data-open=true],[data-state=open] .amt-state-fade,[data-open=true] .amt-state-fade{animation:amt-state-fade-in var(--amt-enter) var(--amt-enter-ease) both}
.amt-state-fade[data-state=closed],.amt-state-fade[data-open=false],[data-state=closed] .amt-state-fade,[data-open=false] .amt-state-fade{animation:amt-state-fade-out var(--amt-exit) var(--amt-exit-ease) both;pointer-events:none}
@keyframes amt-state-fade-in{from{opacity:0}to{opacity:1}}
@keyframes amt-state-fade-out{from{opacity:1}to{opacity:0}}
.amt-state-zoom[data-state=open],.amt-state-zoom[data-open=true],[data-state=open] .amt-state-zoom,[data-open=true] .amt-state-zoom,.amt-state-menu[data-state=open],.amt-state-menu[data-open=true],[data-state=open] .amt-state-menu,[data-open=true] .amt-state-menu{animation:amt-state-zoom-in var(--amt-enter) var(--amt-enter-ease) both}
.amt-state-zoom[data-state=closed],.amt-state-zoom[data-open=false],[data-state=closed] .amt-state-zoom,[data-open=false] .amt-state-zoom,.amt-state-menu[data-state=closed],.amt-state-menu[data-open=false],[data-state=closed] .amt-state-menu,[data-open=false] .amt-state-menu{animation:amt-state-zoom-out var(--amt-exit) var(--amt-exit-ease) both;pointer-events:none}
@keyframes amt-state-zoom-in{from{opacity:0;scale:var(--amt-scale-from)}to{opacity:1;scale:1}}
@keyframes amt-state-zoom-out{from{opacity:1;scale:1}to{opacity:0;scale:var(--amt-scale-from)}}
.amt-state-slide{--amt-slide:0 var(--amt-shift)}
.amt-state-slide[data-side=bottom]{--amt-slide:0 calc(var(--amt-shift) * -1)}
.amt-state-slide[data-side=left]{--amt-slide:var(--amt-shift) 0}
.amt-state-slide[data-side=right]{--amt-slide:calc(var(--amt-shift) * -1) 0}
.amt-state-slide[data-state=open],.amt-state-slide[data-open=true],[data-state=open] .amt-state-slide,[data-open=true] .amt-state-slide{animation:amt-state-slide-in var(--amt-enter) var(--amt-enter-ease) both}
.amt-state-slide[data-state=closed],.amt-state-slide[data-open=false],[data-state=closed] .amt-state-slide,[data-open=false] .amt-state-slide{animation:amt-state-slide-out var(--amt-exit) var(--amt-exit-ease) both;pointer-events:none}
@keyframes amt-state-slide-in{from{opacity:0;translate:var(--amt-slide)}to{opacity:1;translate:none}}
@keyframes amt-state-slide-out{from{opacity:1;translate:none}to{opacity:0;translate:var(--amt-slide)}}
.amt-state-menu{--amt-origin-x:center;--amt-origin-y:top;transform-origin:var(--amt-origin-x) var(--amt-origin-y)}
.amt-state-menu[data-align=start]{--amt-origin-x:left}
.amt-state-menu[data-align=end]{--amt-origin-x:right}
.amt-state-menu[data-side=top]{--amt-origin-y:bottom}
.amt-state-menu[data-side=left]{--amt-origin-x:right;--amt-origin-y:center}
.amt-state-menu[data-side=right]{--amt-origin-x:left;--amt-origin-y:center}
.amt-state-collapse{--amt-enter:.22s;--amt-exit:.18s;--amt-enter-ease:cubic-bezier(.16,1,.3,1);--amt-exit-ease:cubic-bezier(.4,0,1,1);display:grid;grid-template-rows:1fr}
.amt-state-collapse>*{min-height:0;overflow:hidden}
.amt-state-collapse[data-state=open],.amt-state-collapse[data-open=true]{animation:amt-state-collapse-open var(--amt-enter) var(--amt-enter-ease) both}
.amt-state-collapse[data-state=closed],.amt-state-collapse[data-open=false]{animation:amt-state-collapse-close var(--amt-exit) var(--amt-exit-ease) both}
@keyframes amt-state-collapse-open{from{grid-template-rows:0fr;opacity:0}to{grid-template-rows:1fr;opacity:1}}
@keyframes amt-state-collapse-close{from{grid-template-rows:1fr;opacity:1}to{grid-template-rows:0fr;opacity:0}}
@media (prefers-reduced-motion:reduce){
.amt-state-fade,.amt-state-zoom,.amt-state-slide,.amt-state-menu,.amt-state-collapse{--amt-enter:.01ms;--amt-exit:.01ms}
}
"#;

/// Mounts the state-attribute stylesheet.
///
/// Render this once, anywhere above the components you have added an
/// `AMT_`-prefixed class to. The other modules in this crate inject their CSS
/// from the components themselves; these classes go on components this crate
/// does not own, so there is nothing to hang that on — hence the explicit
/// mount. It is deduplicated by key, so rendering it more than once is free,
/// and [`MicroTransitionsStyle`](crate::MicroTransitionsStyle) already includes
/// these rules.
///
/// ```rust, no_run
/// # use dioxus::prelude::*;
/// use dioxus_micro_transitions::primitives::{PrimitivesStyle, AMT_ZOOM};
///
/// fn App() -> Element {
///     rsx! {
///         PrimitivesStyle {}
///         div { class: "{AMT_ZOOM}", "data-state": "open", "Hello" }
///     }
/// }
/// ```
#[component]
pub fn PrimitivesStyle() -> Element {
    rsx! {
        {amt_style!("state", STATE_CSS)}
    }
}
