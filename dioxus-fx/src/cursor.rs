//! Cursor-following effects.
//!
//! [`Spotlight`] is self-contained — it listens to `onmousemove` on its own
//! element. [`MouseFollow`] and [`CursorTrail`] need the pointer position
//! anywhere on the page, which no element-scoped event gives you, so they
//! install one shared `pointermove` listener that writes the position into CSS
//! custom properties on the document root. Mount them once, near the root of
//! your app.

use crate::style::dfx_style;
use dioxus::prelude::*;

pub(crate) const CSS: &[&str] = &[SPOTLIGHT_CSS, MOUSE_FOLLOW_CSS, CURSOR_TRAIL_CSS];

/// Publishes the pointer position as `--dfx-mx` / `--dfx-my` on `:root`, and
/// whether the pointer is over the document as `--dfx-cursor-visible`.
///
/// Guarded so that mounting several cursor components only installs one
/// listener. No-ops outside a browser.
fn use_pointer_variables() {
    use_hook(|| {
        document::eval(
            r#"
            if (!window.__dfxPointer) {
                window.__dfxPointer = true;
                const root = document.documentElement;
                addEventListener('pointermove', (e) => {
                    root.style.setProperty('--dfx-mx', e.clientX + 'px');
                    root.style.setProperty('--dfx-my', e.clientY + 'px');
                    root.style.setProperty('--dfx-cursor-visible', '1');
                }, { passive: true });
                document.addEventListener('mouseleave', () => {
                    root.style.setProperty('--dfx-cursor-visible', '0');
                });
            }
            "#,
        );
    });
}

const SPOTLIGHT_CSS: &str = r#"
.dfx-spotlight{position:relative;overflow:hidden;border:1px solid var(--dfx-track);border-radius:1rem;padding:1.5rem}
.dfx-spotlight__glow{position:absolute;inset:-1px;pointer-events:none;opacity:0;transition:opacity .3s ease}
.dfx-spotlight__content{position:relative;z-index:1}
"#;

/// A panel with a soft glow that tracks the pointer across it.
#[component]
pub fn Spotlight(
    /// Colour of the glow at its centre. Use a translucent colour.
    #[props(default = "rgba(255,255,255,.08)".to_string())]
    glow_color: String,
    /// Radius of the glow, in pixels.
    #[props(default = 250.0)]
    glow_size: f64,
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
        {dfx_style!("spotlight", SPOTLIGHT_CSS)}
        div {
            class: "dfx dfx-spotlight {class}",
            onmousemove: move |evt| {
                let p = evt.element_coordinates();
                point.set((p.x, p.y));
            },
            onmouseenter: move |_| lit.set(true),
            onmouseleave: move |_| lit.set(false),
            ..attributes,
            div {
                class: "dfx-spotlight__glow",
                style: "opacity:{opacity};background:radial-gradient({glow_size}px circle at {x}px {y}px,{glow_color},transparent 80%);",
            }
            div { class: "dfx-spotlight__content", {children} }
        }
    }
}

const MOUSE_FOLLOW_CSS: &str = r#"
.dfx-mouse-follow{position:fixed;top:0;left:0;z-index:9998;pointer-events:none;opacity:var(--dfx-cursor-visible,0);transform:translate(var(--dfx-mx,-100px),var(--dfx-my,-100px)) translate(-50%,-50%);transition:transform var(--dfx-lag) cubic-bezier(.22,1,.36,1),opacity .2s ease}
.dfx-mouse-follow:empty::after{content:"";display:block;width:24px;height:24px;border:1px solid var(--dfx-color);border-radius:9999px;background:color-mix(in srgb,var(--dfx-color) 10%,transparent)}
@media (prefers-reduced-motion:reduce){.dfx-mouse-follow{transition:opacity .2s ease}}
"#;

/// A custom cursor that lags behind the pointer.
///
/// With no children it draws a ring; pass children to use your own shape.
/// Mount once, near the root of your app.
#[component]
pub fn MouseFollow(
    /// How long the element takes to catch up with the pointer, in seconds.
    /// Larger values feel heavier.
    #[props(default = 0.35)]
    lag: f64,
    /// Colour of the default ring. Ignored when you pass your own children.
    #[props(default = "#3b82f6".to_string())]
    color: String,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    use_pointer_variables();
    rsx! {
        {dfx_style!("mouse-follow", MOUSE_FOLLOW_CSS)}
        div {
            class: "dfx dfx-mouse-follow {class}",
            style: "--dfx-lag:{lag}s;--dfx-color:{color};",
            aria_hidden: "true",
            ..attributes,
            {children}
        }
    }
}

const CURSOR_TRAIL_CSS: &str = r#"
.dfx-cursor-trail span{position:fixed;top:0;left:0;z-index:9999;pointer-events:none;border-radius:9999px;background:var(--dfx-color);width:var(--dfx-dot);height:var(--dfx-dot);opacity:calc(var(--dfx-cursor-visible,0) * var(--dfx-fade));transform:translate(var(--dfx-mx,-100px),var(--dfx-my,-100px)) translate(-50%,-50%);transition:transform var(--dfx-lag) cubic-bezier(.22,1,.36,1),opacity .2s ease}
@media (prefers-reduced-motion:reduce){.dfx-cursor-trail span{transition:opacity .2s ease}}
"#;

/// A comet tail of dots chasing the pointer.
///
/// Each dot lags a little further behind and is a little smaller and fainter
/// than the one in front. Mount once, near the root of your app.
#[component]
pub fn CursorTrail(
    /// Diameter of the leading dot, in pixels. The rest taper from there.
    #[props(default = 8.0)]
    size: f64,
    /// How many dots to draw.
    #[props(default = 6)]
    count: usize,
    /// Dot colour. Any CSS colour.
    #[props(default = "#3b82f6".to_string())]
    color: String,
    /// How long the leading dot takes to catch up with the pointer, in seconds.
    #[props(default = 0.12)]
    lag: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    use_pointer_variables();
    let count = count.max(1);
    rsx! {
        {dfx_style!("cursor-trail", CURSOR_TRAIL_CSS)}
        div {
            class: "dfx dfx-cursor-trail {class}",
            style: "--dfx-color:{color};",
            aria_hidden: "true",
            ..attributes,
            for i in 0..count {
                {
                    let taper = 1.0 - i as f64 / count as f64;
                    rsx! {
                        span {
                            key: "{i}",
                            style: "--dfx-dot:{size * taper}px;--dfx-fade:{taper};--dfx-lag:{lag + i as f64 * 0.05}s;",
                        }
                    }
                }
            }
        }
    }
}
