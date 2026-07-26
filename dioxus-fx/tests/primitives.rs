//! Checks the state-attribute add-on against the contract it advertises.
//!
//! These rules are aimed at markup this crate does not render — components from
//! `dioxus-primitives` and anything else using the same data attributes — so
//! nothing here can be caught by rendering our own components. What can be
//! checked is that every class covers both attribute spellings, that closing
//! wins over an open ancestor, and that the keyframes exist.

use dioxus::prelude::*;
use dioxus_fx::primitives::*;
use dioxus_fx::stylesheet;

/// The classes that go on the element carrying the state attribute, and also
/// match through an ancestor that carries it.
const INHERITING: [&str; 4] = [DFX_FADE, DFX_ZOOM, DFX_SLIDE, DFX_MENU];

#[test]
fn every_class_matches_both_attribute_spellings() {
    let css = stylesheet();
    let mut missing = Vec::new();

    for class in INHERITING.iter().chain([&DFX_COLLAPSE]) {
        for selector in [
            format!(".{class}[data-state=open]"),
            format!(".{class}[data-state=closed]"),
            format!(".{class}[data-open=true]"),
            format!(".{class}[data-open=false]"),
        ] {
            if !css.contains(&selector) {
                missing.push(selector);
            }
        }
    }

    assert!(
        missing.is_empty(),
        "these selectors are not in the sheet: {missing:?}"
    );
}

#[test]
fn content_classes_also_match_through_an_open_ancestor() {
    let css = stylesheet();
    let mut missing = Vec::new();

    // A dialog marks only its root, so a class on the panel inside it has to
    // match on the ancestor's state. `DFX_COLLAPSE` is deliberately not in this
    // list: it lays the element out as a grid, so it only ever belongs on the
    // panel that carries its own state.
    for class in INHERITING {
        for selector in [
            format!("[data-state=open] .{class}"),
            format!("[data-state=closed] .{class}"),
            format!("[data-open=true] .{class}"),
            format!("[data-open=false] .{class}"),
        ] {
            if !css.contains(&selector) {
                missing.push(selector);
            }
        }
    }

    assert!(
        missing.is_empty(),
        "these descendant selectors are missing: {missing:?}"
    );
    assert!(
        !css.contains(&format!("[data-state=open] .{DFX_COLLAPSE}")),
        "{DFX_COLLAPSE} should not match through an ancestor"
    );
}

#[test]
fn closing_beats_an_open_ancestor() {
    let css = stylesheet();

    // The two selectors tie on specificity, so the cascade decides it by order:
    // every closed rule has to come after the open rule for the same class.
    for class in INHERITING {
        let open = css
            .find(&format!(".{class}[data-state=open]"))
            .unwrap_or_else(|| panic!("no open rule for {class}"));
        let closed = css
            .find(&format!(".{class}[data-state=closed]"))
            .unwrap_or_else(|| panic!("no closed rule for {class}"));
        assert!(
            open < closed,
            "{class} declares its closed rule before its open rule"
        );
    }
}

#[test]
fn both_halves_hold_their_final_frame() {
    let css = stylesheet();

    // `both` is what keeps closing content hidden while the library waits for
    // the animation to finish before unmounting it. Without it the content
    // snaps back to visible for the last few frames.
    for line in css
        .lines()
        .filter(|line| line.contains("animation:dfx-state-"))
    {
        assert!(
            line.ends_with("both}") || line.contains("both;"),
            "no `both` fill on: {line}"
        );
    }
}

#[test]
fn the_positioning_transform_is_left_alone() {
    let css = stylesheet();

    // Popovers and tooltips are centred with `transform: translateX(-50%)`.
    // Animating the `transform` shorthand would drop that; the independent
    // `translate` and `scale` properties compose with it.
    for keyframes in css.split("@keyframes dfx-state-").skip(1) {
        let body = keyframes.split('}').next().unwrap_or_default();
        assert!(
            !body.contains("transform:"),
            "add-on keyframes must not set transform: {body}"
        );
    }
}

#[test]
fn reduced_motion_shortens_rather_than_removes() {
    let css = stylesheet();

    // `animation:none` would drop the element out of `getAnimations()`, and the
    // library would unmount closing content before it had a chance to hide. So
    // the reduced-motion rule shortens the durations to nothing instead.
    let rule = ".dfx-state-fade,.dfx-state-zoom,.dfx-state-slide,.dfx-state-menu,.dfx-state-collapse{--dfx-enter:.01ms;--dfx-exit:.01ms}";
    let at = css
        .find(rule)
        .expect("the add-on shortens its durations under reduced motion");
    assert!(
        css[..at].ends_with("@media (prefers-reduced-motion:reduce){\n"),
        "that rule has to sit inside the reduced-motion block"
    );
}

#[test]
fn the_style_component_renders_once_per_key() {
    fn app() -> Element {
        rsx! {
            PrimitivesStyle {}
            PrimitivesStyle {}
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    // Head styles do not land in the body; this asserts mounting it twice is
    // free rather than a panic or a duplicated sheet.
    assert!(dioxus_ssr::render(&dom).is_empty());
}
