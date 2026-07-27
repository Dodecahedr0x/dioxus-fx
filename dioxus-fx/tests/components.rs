//! Renders every component through the SSR renderer and checks the output.
//!
//! The point is coverage rather than depth: with 150+ components, the failure
//! mode that matters is one of them silently rendering nothing, losing its
//! stylesheet, or referencing a class that no rule defines.

use dioxus::prelude::*;
use dioxus_fx::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

/// Asserts a component renders markup carrying the crate's base class.
macro_rules! assert_renders {
    ($label:expr, $body:expr) => {{
        fn app() -> Element {
            rsx! { {$body} }
        }
        let html = render(app);
        assert!(!html.is_empty(), "{} rendered nothing", $label);
        assert!(
            html.contains("class=\"dfx "),
            "{} is missing the base `dfx` class: {html}",
            $label
        );
        html
    }};
}

/// Every loading component takes only defaulted props, so they can all be
/// rendered the same way.
macro_rules! loading_components {
    ($($name:ident),* $(,)?) => {
        const LOADING_COUNT: usize = [$(stringify!($name)),*].len();

        #[test]
        fn every_loading_component_renders() {
            $(
                assert_renders!(
                    stringify!($name),
                    rsx! { dioxus_fx::loading::$name {} }
                );
            )*
        }
    };
}

loading_components! {
    AccordionLoader, AppIconLoad, AppleBreathe, AppleEqualizer, AppleIconMorph, ApplePulseDots,
    AppleScalePulse, AppleSoundWave, AppleTextReveal, AppleUnlock, ArcTracer, BarCascade,
    BarSweep, BobbingDots, BounceDots, BouncingBars, BouncingDots, BouncingLines,
    BouncingSquare, BreatheRing, BreathingGlow, BreathingSquare, CircularBars, ClassicSpinner,
    ClockSpinner, CometSpinner, ConcentricPulse, ConcentricRing, ConcentricSquares, ConveyorLoop,
    CrossSpinner, CubeFlipSpring, DashRing, DashedSpiral, DiamondGrid, DiamondRotateSpring,
    DotSpinner, DotsRing, DoubleRing, DropDot, DualArc, DynamicIsland,
    ElasticBars, ElasticSquare, ExpandingCross, FaceIdScan, FadeArc, FadeDots,
    FlipSquare, FloatingDiamonds, FluidBars, FluidDiamond, FluidDotOrbit, FluidSkeleton,
    Gears, GlassmorphicCard, GradientArc, GridDots, HapticRing, Heartbeat,
    HexagonSpinner, Hourglass, InfinityPath, IntersectingRings, IosSpinner, LineSpinner,
    LiquidDots, MacTerminal, MagneticDots, MinimalTriangle, MorphDotRing, MorphLoader,
    MorphingBars, MorphingInfinity, MorphingRing, MorphingShape, NewtonsCradle, OffsetRings,
    OrbitingCircles, OrbitingDot, OrigamiShape, Pendulum, PulsatingDots, PulseDot,
    PulseDots, PulseSquare, Pulse, PumpingHeart, RadarSweep, RingSweep,
    RippleEffect, RotatingCross, RotatingTriangle, ShapeShiftGrid, ShimmerLine, SiriWave,
    SkeletonLoader, Skeleton, SlidingBars, SmoothDotShift, SmoothRing, SmoothRoundedSquare,
    SpinningSquares, SpiralSpinner, SpringBars, SpringDotMatrix, SpringHexagon, SpringRingExpand,
    SpringTextPop, SquareAccordion, SquareGrid, SquareSnake, SquareSpinner, StackedBarPulse,
    SwappingDots, SwirlingSpinner, SymmetricWave, TerminalLoader, TextBlink, TextDots,
    TextMorph, TextShimmerWave, TextShimmer, TrailingDots, TripleDotSpinner, TwinOrbit,
    TypingIndicator, Typing, WanderingCube, WatchSpinner, WaveDots, WavePhysicsLoader,
    WaveformLoader, ZigZagPulse,
}

#[test]
fn the_whole_loading_set_is_present() {
    assert_eq!(LOADING_COUNT, 134);
}

#[test]
fn entrance_components_render() {
    assert_renders!("FadeIn", rsx! { FadeIn { "x" } });
    assert_renders!("FadeUp", rsx! { FadeUp { "x" } });
    assert_renders!("FadeDown", rsx! { FadeDown { "x" } });
    assert_renders!("SlideLeft", rsx! { SlideLeft { "x" } });
    assert_renders!("SlideRight", rsx! { SlideRight { "x" } });
    assert_renders!("ScaleIn", rsx! { ScaleIn { "x" } });
    assert_renders!("ZoomIn", rsx! { ZoomIn { "x" } });
}

#[test]
fn text_components_render() {
    assert_renders!("BlurText", rsx! { BlurText { text: "hi" } });
    assert_renders!("CharacterStagger", rsx! { CharacterStagger { text: "hi" } });
    assert_renders!("WordReveal", rsx! { WordReveal { text: "hi there" } });
    assert_renders!("TextReveal", rsx! { TextReveal { text: "one\ntwo" } });
}

#[test]
fn scroll_components_render() {
    assert_renders!("ProgressIndicator", rsx! { ProgressIndicator {} });
    assert_renders!("ScrollReveal", rsx! { ScrollReveal { "x" } });
    assert_renders!(
        "StickyReveal",
        rsx! {
            StickyReveal { items: vec![StickyRevealItem::new("Title", "Body")] }
        }
    );
}

#[test]
fn cursor_components_render() {
    assert_renders!("Spotlight", rsx! { Spotlight { "x" } });
    assert_renders!("MouseFollow", rsx! { MouseFollow {} });
    assert_renders!("CursorTrail", rsx! { CursorTrail {} });
}

#[test]
fn hover_components_render() {
    assert_renders!("GlowButton", rsx! { GlowButton { "Go" } });
    assert_renders!("MagneticButton", rsx! { MagneticButton { "Go" } });
    assert_renders!("TiltCard", rsx! { TiltCard { "Go" } });
    assert_renders!(
        "CardHover",
        rsx! {
            CardHover { items: vec![CardHoverItem::new("Title", "Body")] }
        }
    );
}

#[test]
fn surface_components_render() {
    assert_renders!("Frost", rsx! { Frost { "x" } });
    assert_renders!("Lens", rsx! { Lens { "x" } });
    assert_renders!("Ripple", rsx! { Ripple { "x" } });
    assert_renders!("Vhs", rsx! { Vhs { "x" } });
    assert_renders!("Glitch", rsx! { Glitch { "x" } });
    assert_renders!("Blaze", rsx! { Blaze { "x" } });
    assert_renders!("Halftone", rsx! { Halftone { "x" } });
    assert_renders!("Bubble", rsx! { Bubble { "x" } });
    assert_renders!("Mist", rsx! { Mist { "x" } });
    assert_renders!("Droplets", rsx! { Droplets { "x" } });
    assert_renders!("Tiles", rsx! { Tiles { "x" } });
    assert_renders!("Honeycomb", rsx! { Honeycomb { "x" } });
    assert_renders!("Laser", rsx! { Laser { "x" } });
    assert_renders!("Shatter", rsx! { Shatter { "x" } });
    assert_renders!("Stipple", rsx! { Stipple { "x" } });
    assert_renders!("Liquid", rsx! { Liquid { "x" } });
    assert_renders!("Dissolve", rsx! { Dissolve { "x" } });
    assert_renders!("Bend", rsx! { Bend { "x" } });
    assert_renders!("GlassShape", rsx! { GlassShape {} });
    assert_renders!("DitherShape", rsx! { DitherShape {} });
    assert_renders!("ParticleShape", rsx! { ParticleShape {} });
    assert_renders!("Ascii", rsx! { Ascii { "x" } });
    assert_renders!("Cloth", rsx! { Cloth { "x" } });
    assert_renders!(
        "Peel",
        rsx! {
            Peel { beneath: rsx! { "under" }, "over" }
        }
    );
}

#[test]
fn the_tile_grids_emit_every_cell_they_are_asked_for() {
    fn app() -> Element {
        rsx! {
            Tiles { columns: 4, rows: 3, "x" }
        }
    }
    assert_eq!(render(app).matches("dfx-tiles__tile").count(), 12);
}

#[test]
fn the_honeycomb_overhangs_its_lattice_by_one_column() {
    // The half-cell shift on odd rows would leave a notch down one side, so
    // every row renders one cell past the edge for the clip to swallow.
    fn app() -> Element {
        rsx! {
            Honeycomb { columns: 5, rows: 4, "x" }
        }
    }
    assert_eq!(render(app).matches("dfx-honeycomb__cell").count(), 4 * 6);
}

#[test]
fn shatter_starts_centred_and_moves_to_where_it_was_struck() {
    fn app() -> Element {
        rsx! {
            Shatter { "x" }
        }
    }
    let html = render(app);
    // A percentage, because until a click there is no measured impact point —
    // and no flash, whose `translate` could not resolve one.
    assert!(html.contains("--dfx-x:50%;--dfx-y:50%;"), "{html}");
    assert!(!html.contains("dfx-shatter__flash"), "{html}");
}

#[test]
fn the_shape_components_quote_their_silhouette_safely() {
    // `src` lands in a CSS `url('…')` inside an HTML style attribute, so the
    // built-in silhouette has to survive both parsers: percent-encoded down to
    // characters that are neither a quote, a space nor a parenthesis.
    fn app() -> Element {
        rsx! {
            GlassShape {}
            DitherShape {}
            ParticleShape {}
        }
    }
    // The renderer escapes the CSS quotes to `&#39;` on the way into the
    // attribute; the browser puts them back before the CSS parser sees them.
    const OPEN: &str = "--dfx-shape:url(&#39;";
    let html = render(app);
    for chunk in html.split(OPEN).skip(1) {
        let src = chunk.split("&#39;").next().expect("the url is closed");
        assert!(
            !src.contains([' ', '"', '\'', '(', ')']),
            "silhouette would break out of url('…'): {src}"
        );
    }
    assert_eq!(html.matches(OPEN).count(), 3, "{html}");
}

#[test]
fn the_scroll_driven_effects_rest_where_their_content_is_readable() {
    // Both are guarded behind `@supports (animation-timeline:view())`, so a
    // browser without scroll-driven animations never starts them. Whatever they
    // render in that case is what those users get permanently, which rules out
    // any resting state that hides the content.
    let css = stylesheet();
    for guarded in [
        ".dfx-dissolve{animation:",
        ".dfx-bend .dfx-surface__content{",
    ] {
        let before = css.split(guarded).next().expect("the rule is present");
        assert!(
            before.ends_with("@supports (animation-timeline:view()){\n"),
            "{guarded} is not behind a scroll-timeline @supports guard"
        );
    }
    // Dissolve's grain rests at zero — solid content — rather than at one.
    assert!(
        css.contains("@property --dfx-grain{syntax:\"<number>\";inherits:true;initial-value:0}"),
        "Dissolve would rest dissolved"
    );
}

#[test]
fn laser_leaves_its_children_visible_once_the_scan_has_passed() {
    // Every other effect here is additive, so `prefers-reduced-motion` can stop
    // it dead. This one masks its children away ahead of the beam, and a
    // stopped animation would leave them masked away for good.
    let css = stylesheet();
    let reduced = css
        .split("@media (prefers-reduced-motion:reduce)")
        .skip(1)
        // The block that opens on the Laser's own rules, not the base block
        // further up whose tail happens to run past them.
        .find(|block| block.trim_start().starts_with("{\n.dfx-laser"))
        .expect("Laser declares its own reduced-motion behaviour");
    // Only the media query's own closing brace starts a line; every rule inside
    // it closes at the end of one.
    let block = reduced.split_once("\n}").expect("the block is closed").0;
    assert!(block.contains("mask-image:none"), "{block}");
}

#[test]
fn surface_effects_wrap_their_children_without_swallowing_them() {
    fn app() -> Element {
        rsx! {
            Vhs {
                a { href: "https://example.com", "link" }
            }
        }
    }
    let html = render(app);
    // The point of the module: the content is still real markup under the
    // overlay, not a picture of it.
    assert!(html.contains("href=\"https://example.com\""), "{html}");
    assert!(html.contains("dfx-surface__content"), "{html}");
    assert!(html.contains("dfx-surface__layer"), "{html}");
}

#[test]
fn surface_intensity_is_clamped_to_the_documented_range() {
    fn app() -> Element {
        rsx! {
            Blaze { intensity: 4.0, "x" }
            Halftone { intensity: -1.0, "x" }
        }
    }
    let html = render(app);
    assert!(html.contains("--dfx-intensity:1;"), "{html}");
    assert!(html.contains("--dfx-intensity:0;"), "{html}");
}

#[test]
fn every_peel_corner_selects_its_own_geometry() {
    const ALL: [PeelCorner; 4] = [
        PeelCorner::TopRight,
        PeelCorner::TopLeft,
        PeelCorner::BottomRight,
        PeelCorner::BottomLeft,
    ];

    fn app() -> Element {
        rsx! {
            for corner in ALL {
                Peel { corner, beneath: rsx! { "under" }, "over" }
            }
        }
    }
    let html = render(app);
    for slug in [
        "dfx-peel--tr",
        "dfx-peel--tl",
        "dfx-peel--br",
        "dfx-peel--bl",
    ] {
        assert!(html.contains(slug), "no peel rendered {slug}: {html}");
    }
    assert_eq!(PeelCorner::default(), PeelCorner::TopRight);
}

#[test]
fn every_button_interaction_renders() {
    fn app() -> Element {
        const ALL: [ButtonInteraction; 12] = [
            ButtonInteraction::SlideArrow,
            ButtonInteraction::Sparkle,
            ButtonInteraction::Morph,
            ButtonInteraction::ColorMorph,
            ButtonInteraction::Pulse,
            ButtonInteraction::Rotate,
            ButtonInteraction::Shake,
            ButtonInteraction::Ring,
            ButtonInteraction::Glare,
            ButtonInteraction::TextReveal,
            ButtonInteraction::Magnetic,
            ButtonInteraction::ExpandRing,
        ];
        rsx! {
            for interaction in ALL {
                AnimatedButton { label: "Go", interaction }
            }
        }
    }
    let html = render(app);
    // Morph and ColorMorph deliberately share one variant, so eleven classes
    // cover all twelve interactions.
    for slug in [
        "slide-arrow",
        "sparkle",
        "morph",
        "pulse",
        "rotate",
        "shake",
        "ring",
        "glare",
        "text-reveal",
        "magnetic",
        "expand-ring",
    ] {
        assert!(
            html.contains(&format!("dfx-abtn--{slug}")),
            "no button rendered the {slug} variant: {html}"
        );
    }
}

#[test]
fn focus_blur_links_render() {
    fn app() -> Element {
        rsx! {
            FocusBlurLinks { items: vec![("@x".to_string(), "https://example.com".to_string())] }
        }
    }
    let html = render(app);
    assert!(html.contains("dfx-focus-blur"), "{html}");
    assert!(html.contains("https://example.com"), "{html}");
}

#[test]
fn every_card_spread_layout_renders_its_full_stack() {
    const ALL: [CardSpreadLayout; 9] = [
        CardSpreadLayout::Arc5,
        CardSpreadLayout::Arc7,
        CardSpreadLayout::LongArc5,
        CardSpreadLayout::LinearSpread,
        CardSpreadLayout::CornerFan,
        CardSpreadLayout::StampArc,
        CardSpreadLayout::CascadeStagger,
        CardSpreadLayout::ScatterSpread,
        CardSpreadLayout::WheelFan,
    ];

    fn app() -> Element {
        rsx! {
            for layout in ALL {
                CardSpread { layout }
            }
        }
    }
    let html = render(app);
    let expected: usize = ALL.iter().map(|l| l.count()).sum();
    assert_eq!(
        html.matches("--dfx-tx").count(),
        expected,
        "wrong number of cards across the nine layouts: {html}"
    );
    assert_eq!(CardSpreadLayout::Arc7.count(), 7);
}

/// Three photos, shared by the carousel tests.
fn sample_items() -> Vec<CardItem> {
    vec![
        CardItem::new("a.jpg", "Alpha").with_date("Today"),
        CardItem::new("b.jpg", "Beta").with_date("1d ago"),
        CardItem::new("c.jpg", "Gamma").with_date("1w ago"),
    ]
}

#[test]
fn carousels_render_and_mark_the_active_slide() {
    fn carousel() -> Element {
        rsx! {
            CardCarousel { items: sample_items(), initial_index: 1 }
        }
    }
    let html = render(carousel);
    assert!(html.contains("dfx-carousel__slide dfx-active"), "{html}");
    assert!(html.contains("--dfx-active:1"), "{html}");

    fn cover_flow() -> Element {
        rsx! {
            CardCoverFlow { items: sample_items(), initial_index: 1 }
        }
    }
    let html = render(cover_flow);
    assert!(html.contains("dfx-cover-flow__slide dfx-active"), "{html}");

    fn time_machine() -> Element {
        rsx! {
            CardTimeMachine { items: sample_items() }
        }
    }
    let html = render(time_machine);
    assert!(html.contains("dfx-time-machine__card dfx-active"), "{html}");
    // The caption tracks the active photo.
    assert!(html.contains("Alpha"), "{html}");
}

#[test]
fn an_out_of_range_initial_index_is_clamped() {
    fn app() -> Element {
        rsx! {
            CardCarousel { items: vec![CardItem::new("a.jpg", "A")], initial_index: 99 }
        }
    }
    let html = render(app);
    assert!(html.contains("--dfx-active:0"), "{html}");
}

#[test]
fn props_reach_the_rendered_custom_properties() {
    fn app() -> Element {
        rsx! {
            Pulse { size: 20.0, color: "rebeccapurple", duration: 3.0 }
        }
    }
    let html = render(app);
    assert!(html.contains("--dfx-size:20px"), "{html}");
    assert!(html.contains("--dfx-color:rebeccapurple"), "{html}");
    assert!(html.contains("--dfx-duration:3s"), "{html}");
}

#[test]
fn extra_classes_and_attributes_pass_through() {
    fn app() -> Element {
        rsx! {
            IosSpinner { class: "my-spinner", id: "busy" }
        }
    }
    let html = render(app);
    assert!(html.contains("my-spinner"), "{html}");
    assert!(html.contains("busy"), "{html}");
}

#[test]
fn loaders_announce_themselves() {
    fn app() -> Element {
        rsx! { RingSweep {} }
    }
    let html = render(app);
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(html.contains("aria-label=\"Loading\""), "{html}");
}
