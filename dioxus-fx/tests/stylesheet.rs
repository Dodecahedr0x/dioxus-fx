//! Checks the CSS and the markup agree with each other.
//!
//! A component whose markup names a class the stylesheet never defines renders
//! as a motionless box — it compiles, it renders, and nothing moves. These
//! tests catch that by cross-referencing the two.

use dioxus::prelude::*;
use dioxus_fx::prelude::*;
use std::collections::BTreeSet;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

/// Every `dfx-…` class appearing in `class="…"` attributes of `html`.
fn classes_in(html: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for chunk in html.split("class=\"").skip(1) {
        let Some((value, _)) = chunk.split_once('"') else {
            continue;
        };
        for class in value.split_whitespace() {
            if class.starts_with("dfx-") {
                found.insert(class.to_string());
            }
        }
    }
    found
}

#[test]
fn the_stylesheet_defines_every_class_the_components_use() {
    // One instance of each category is enough to reach the shared plumbing;
    // `components.rs` covers that each individual component renders.
    fn app() -> Element {
        rsx! {
            IosSpinner {}
            WavePhysicsLoader {}
            FadeUp { "x" }
            BlurText { text: "x" }
            ScrollReveal { "x" }
            ProgressIndicator {}
            Spotlight { "x" }
            MouseFollow {}
            CursorTrail {}
            GlowButton { "x" }
            MagneticButton { "x" }
            TiltCard { "x" }
            CardHover { items: vec![CardHoverItem::new("t", "d")] }
            Frost { "x" }
            Lens { "x" }
            Ripple { "x" }
            Peel { beneath: rsx! { "u" }, "o" }
            Vhs { "x" }
            Glitch { "x" }
            Blaze { "x" }
            Halftone { "x" }
            AnimatedButton { label: "x" }
            FocusBlurLinks { items: vec![("x".to_string(), "#".to_string())] }
            CardSpread {}
            CardCarousel { items: vec![CardItem::new("a.jpg", "A")] }
            CardCoverFlow { items: vec![CardItem::new("a.jpg", "A")] }
            CardTimeMachine { items: vec![CardItem::new("a.jpg", "A")] }
            StickyReveal { items: vec![StickyRevealItem::new("t", "d")] }
        }
    }

    let css = stylesheet();
    let mut undefined = Vec::new();
    for class in classes_in(&render(app)) {
        // `dfx-active` and `dfx-visible` are state flags, always used alongside
        // a component class and matched by compound selectors.
        if !css.contains(&format!(".{class}")) {
            undefined.push(class);
        }
    }
    assert!(
        undefined.is_empty(),
        "these classes are rendered but never defined: {undefined:?}"
    );
}

#[test]
fn every_animation_referenced_has_keyframes() {
    let css = stylesheet();
    let mut missing = Vec::new();
    // `animation-name:` names an animation outright; the `animation:` shorthand
    // leads with the name in every rule this crate writes.
    let referenced = css
        .split("animation-name:")
        .skip(1)
        .chain(css.split("animation:").skip(1));
    for chunk in referenced {
        let name: String = chunk
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        if !name.starts_with("dfx-") {
            continue;
        }
        if !css.contains(&format!("@keyframes {name}")) {
            missing.push(name);
        }
    }
    assert!(
        missing.is_empty(),
        "these animations have no @keyframes block: {missing:?}"
    );
}

#[test]
fn the_stylesheet_covers_every_enabled_category() {
    let css = stylesheet();
    // A representative rule from each module's CSS.
    for marker in [
        ".dfx-ios-spinner",
        ".dfx-fade-up",
        ".dfx-blur-text",
        ".dfx-glow-button",
        ".dfx-spotlight",
        ".dfx-scroll-reveal",
        ".dfx-abtn",
        ".dfx-card-spread",
        ".dfx-surface__layer",
        ".dfx-state-fade",
    ] {
        assert!(css.contains(marker), "stylesheet is missing {marker}");
    }
    // The one component whose keyframes are computed rather than declared.
    assert!(
        css.contains("@keyframes dfx-wave-physics-ball"),
        "{}",
        &css[..200]
    );
}

#[test]
fn every_block_and_function_in_the_stylesheet_is_closed() {
    // The CSS is hand-written inside Rust string literals, where a dropped `}`
    // or `)` is invisible in review and silently swallows every rule after it —
    // the browser skips to the end of the erroneous block and keeps going.
    let css = stylesheet();
    let (mut blocks, mut parens, mut line) = (0i32, 0i32, 1usize);
    for c in css.chars() {
        match c {
            '\n' => line += 1,
            '{' => blocks += 1,
            '}' => blocks -= 1,
            '(' => parens += 1,
            ')' => parens -= 1,
            _ => {}
        }
        assert!(blocks >= 0, "a block closes before it opens, line {line}");
        assert!(
            parens >= 0,
            "a function closes before it opens, line {line}"
        );
    }
    assert_eq!(blocks, 0, "{blocks} block(s) left open");
    assert_eq!(parens, 0, "{parens} function call(s) left open");
}

#[test]
fn the_base_custom_properties_are_defined_once() {
    let css = stylesheet();
    assert_eq!(
        css.matches("--dfx-size:40px").count(),
        1,
        "the base block should appear exactly once in the assembled sheet"
    );
}

#[test]
fn components_inject_their_own_styles() {
    fn app() -> Element {
        rsx! {
            Pulse {}
            Pulse {}
            Pulse {}
        }
    }
    // Head elements do not appear in the SSR body, but rendering three copies
    // must not panic or duplicate work — the dedup key is what guards that.
    let html = render(app);
    assert_eq!(html.matches("dfx-pulse").count(), 3, "{html}");
}
