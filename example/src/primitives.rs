//! Stories for `dioxus_fx::primitives`, the state-attribute
//! add-on.
//!
//! These drive real [`dioxus_primitives`] components — the crate behind
//! <https://dioxuslabs.com/components> — with nothing added but a class, which
//! is the whole claim the module makes. The primitives are unstyled, so each
//! story brings enough CSS to see the shape it is animating.

use crate::num;
use dioxus::prelude::*;
use dioxus_fx::primitives::*;
use dioxus_primitives::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use dioxus_primitives::dialog::{DialogContent, DialogRoot, DialogTitle};
use dioxus_primitives::popover::{PopoverContent, PopoverRoot, PopoverTrigger};
use dioxus_primitives::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use dioxus_primitives::{ContentAlign, ContentSide};
use dioxus_showcase::prelude::*;

/// The minimum styling the unstyled primitives need to be visible at all.
///
/// Deliberately plain: the point of each story is the motion, not the theme.
const DEMO_CSS: &str = r#"
.demo-trigger{padding:8px 14px;border:1px solid currentColor;border-radius:8px;background:none;color:inherit;font:inherit;cursor:pointer}
.demo-panel{padding:16px 20px;border:1px solid rgba(128,128,128,.4);border-radius:12px;background:Canvas;color:CanvasText;box-shadow:0 12px 32px rgba(0,0,0,.18)}
.demo-backdrop{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,.4);z-index:10}
.demo-popover{position:absolute;top:100%;left:50%;transform:translateX(-50%);margin-top:8px;z-index:10}
.demo-anchor{position:relative;display:inline-block}
.demo-accordion{width:280px;text-align:left}
.demo-accordion button{width:100%;padding:10px 0;border:0;border-bottom:1px solid rgba(128,128,128,.3);background:none;color:inherit;font:inherit;text-align:left;cursor:pointer}
/* The direct child of an DFX_COLLAPSE panel is what gets clipped, so its own
   padding would survive the collapse. Space the content inside it instead. */
.demo-accordion p{margin:0;padding:10px 0}

"#;

/// Mounts the add-on stylesheet and the demo styling.
#[component]
fn Demo(children: Element) -> Element {
    rsx! {
        PrimitivesStyle {}
        document::Style { href: "dfx:primitives-demo", {DEMO_CSS} }
        {children}
    }
}

/// A modal dialog: the backdrop fades while the panel inside it zooms.
///
/// The class on `DialogRoot` is what keeps the dialog mounted long enough to
/// animate out; the one on `DialogContent` matches through its open ancestor.
#[story(title = "Primitives/Dialog", tags = ["primitives"])]
pub fn dialog(enter: f64, exit: f64) -> Element {
    let mut open = use_signal(|| false);
    let timing = format!(
        "--dfx-enter:{}s;--dfx-exit:{}s;",
        num(enter, 0.18),
        num(exit, 0.14)
    );

    rsx! {
        Demo {
            button { class: "demo-trigger", onclick: move |_| open.set(true), "Open dialog" }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                class: "demo-backdrop {DFX_FADE}",
                style: "{timing}",
                DialogContent { class: "demo-panel {DFX_ZOOM}",
                    DialogTitle { "Zoomed in" }
                    p { "The backdrop fades, the panel scales up behind it." }
                    button {
                        class: "demo-trigger",
                        onclick: move |_| open.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}

/// A popover sliding out of its trigger, in the direction `data-side` implies.
#[story(title = "Primitives/Popover", tags = ["primitives"])]
pub fn popover(shift: f64, enter: f64) -> Element {
    let mut open = use_signal(|| false);
    let timing = format!(
        "--dfx-shift:{}px;--dfx-enter:{}s;",
        num(shift, 6.0),
        num(enter, 0.18)
    );

    rsx! {
        Demo {
            div { class: "demo-anchor",
                PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
                    PopoverTrigger { class: "demo-trigger", "Open popover" }
                    PopoverContent {
                        side: ContentSide::Bottom,
                        align: ContentAlign::Center,
                        class: "demo-panel demo-popover {DFX_SLIDE}",
                        style: "{timing}",
                        "Slid down out of the trigger."
                    }
                }
            }
        }
    }
}

/// A tooltip on hover, sliding up from its trigger.
#[story(title = "Primitives/Tooltip", tags = ["primitives"])]
pub fn tooltip(shift: f64) -> Element {
    let timing = format!("--dfx-shift:{}px;", num(shift, 6.0));

    rsx! {
        Demo {
            div { class: "demo-anchor",
                Tooltip {
                    TooltipTrigger { class: "demo-trigger", "Hover me" }
                    TooltipContent {
                        side: ContentSide::Top,
                        class: "demo-panel {DFX_SLIDE}",
                        style: "{timing}",
                        "Tooltips get the same class."
                    }
                }
            }
        }
    }
}

/// A menu-style popover growing out of the trigger corner it is aligned to.
#[story(title = "Primitives/Menu", tags = ["primitives"])]
pub fn menu(enter: f64) -> Element {
    let mut open = use_signal(|| false);
    let timing = format!("--dfx-enter:{}s;", num(enter, 0.18));

    rsx! {
        Demo {
            div { class: "demo-anchor",
                PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
                    PopoverTrigger { class: "demo-trigger", "Open menu" }
                    PopoverContent {
                        side: ContentSide::Bottom,
                        align: ContentAlign::Start,
                        class: "demo-panel demo-popover {DFX_MENU}",
                        style: "{timing}",
                        div { "Rename" }
                        div { "Duplicate" }
                        div { "Delete" }
                    }
                }
            }
        }
    }
}

/// Accordion panels collapsing to the height they actually measure.
#[story(title = "Primitives/Accordion", tags = ["primitives"])]
pub fn accordion(enter: f64) -> Element {
    let timing = format!("--dfx-enter:{}s;", num(enter, 0.22));

    rsx! {
        Demo {
            Accordion { class: "demo-accordion", allow_multiple_open: false, collapsible: true,
                for (i , (title , body)) in [
                    ("What is animating?", "`grid-template-rows` between 0fr and 1fr, so no height has to be known up front."),
                    ("Why not max-height?", "A guessed max-height either clips the content or drags out the close."),
                ]
                    .into_iter()
                    .enumerate()
                {
                    AccordionItem { key: "{i}", index: i, default_open: i == 0,
                        AccordionTrigger { "{title}" }
                        AccordionContent { class: "{DFX_COLLAPSE}", style: "{timing}",
                            div {
                                p { "{body}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
