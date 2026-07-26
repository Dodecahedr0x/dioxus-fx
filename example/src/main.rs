//! A browsable gallery of every component in `dioxus-micro-transitions`.
//!
//! Run it with `dx serve --package gallery`.

use dioxus::prelude::*;
use dioxus_micro_transitions::prelude::*;

fn main() {
    dioxus::launch(App);
}

/// Renders one component under its name.
#[component]
fn Tile(label: String, children: Element) -> Element {
    rsx! {
        figure { class: "tile",
            div { class: "stage", {children} }
            figcaption { "{label}" }
        }
    }
}

/// One titled block of the gallery.
#[component]
fn Section(title: String, note: String, children: Element) -> Element {
    rsx! {
        section {
            h2 { "{title}" }
            p { class: "note", "{note}" }
            div { class: "grid", {children} }
        }
    }
}

/// Builds a tile per named loading component, so the gallery cannot drift out
/// of sync with the module.
macro_rules! loaders {
    ($($name:ident)*) => {
        rsx! {
            $(
                Tile { label: stringify!($name),
                    dioxus_micro_transitions::loading::$name {}
                }
            )*
        }
    };
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { href: "gallery", {GALLERY_CSS} }
        ProgressIndicator {}
        CursorTrail { size: 10.0 }

        header {
            h1 { "dioxus-micro-transitions" }
            p { "155 Amicro components, ported to Dioxus. No animation runtime, no CSS framework." }
        }

        Section {
            title: "Loading",
            note: "134 loaders. Each takes size, color and duration; colour defaults to currentColor.",
            {loaders! {
        AccordionLoader AppIconLoad AppleBreathe AppleEqualizer AppleIconMorph
        ApplePulseDots AppleScalePulse AppleSoundWave AppleTextReveal AppleUnlock
        ArcTracer BarCascade BarSweep BobbingDots BounceDots
        BouncingBars BouncingDots BouncingLines BouncingSquare BreatheRing
        BreathingGlow BreathingSquare CircularBars ClassicSpinner ClockSpinner
        CometSpinner ConcentricPulse ConcentricRing ConcentricSquares ConveyorLoop
        CrossSpinner CubeFlipSpring DashRing DashedSpiral DiamondGrid
        DiamondRotateSpring DotSpinner DotsRing DoubleRing DropDot
        DualArc DynamicIsland ElasticBars ElasticSquare ExpandingCross
        FaceIdScan FadeArc FadeDots FlipSquare FloatingDiamonds
        FluidBars FluidDiamond FluidDotOrbit FluidSkeleton Gears
        GlassmorphicCard GradientArc GridDots HapticRing Heartbeat
        HexagonSpinner Hourglass InfinityPath IntersectingRings IosSpinner
        LineSpinner LiquidDots MacTerminal MagneticDots MinimalTriangle
        MorphDotRing MorphLoader MorphingBars MorphingInfinity MorphingRing
        MorphingShape NewtonsCradle OffsetRings OrbitingCircles OrbitingDot
        OrigamiShape Pendulum PulsatingDots PulseDot PulseDots
        PulseSquare Pulse PumpingHeart RadarSweep RingSweep
        RippleEffect RotatingCross RotatingTriangle ShapeShiftGrid ShimmerLine
        SiriWave SkeletonLoader Skeleton SlidingBars SmoothDotShift
        SmoothRing SmoothRoundedSquare SpinningSquares SpiralSpinner SpringBars
        SpringDotMatrix SpringHexagon SpringRingExpand SpringTextPop SquareAccordion
        SquareGrid SquareSnake SquareSpinner StackedBarPulse SwappingDots
        SwirlingSpinner SymmetricWave TerminalLoader TextBlink TextDots
        TextMorph TextShimmerWave TextShimmer TrailingDots TripleDotSpinner
        TwinOrbit TypingIndicator Typing WanderingCube WatchSpinner
        WaveDots WavePhysicsLoader WaveformLoader ZigZagPulse
            }}
        }

        Section { title: "Entrance", note: "Play once, on mount.",
            Tile { label: "FadeIn",
                FadeIn { Swatch {} }
            }
            Tile { label: "FadeUp",
                FadeUp { Swatch {} }
            }
            Tile { label: "FadeDown",
                FadeDown { Swatch {} }
            }
            Tile { label: "SlideLeft",
                SlideLeft { Swatch {} }
            }
            Tile { label: "SlideRight",
                SlideRight { Swatch {} }
            }
            Tile { label: "ScaleIn",
                ScaleIn { Swatch {} }
            }
            Tile { label: "ZoomIn",
                ZoomIn { Swatch {} }
            }
        }

        Section { title: "Text", note: "Staggered reveals by character, word or line.",
            Tile { label: "BlurText",
                BlurText { text: "Blur reveal" }
            }
            Tile { label: "CharacterStagger",
                CharacterStagger { text: "Character stagger" }
            }
            Tile { label: "WordReveal",
                WordReveal { text: "One word at a time" }
            }
            Tile { label: "TextReveal",
                TextReveal { text: "Line by line\nfrom behind a mask" }
            }
        }

        Section { title: "Hover", note: "React to the pointer over their own element.",
            Tile { label: "GlowButton",
                GlowButton { "Hover me" }
            }
            Tile { label: "MagneticButton",
                MagneticButton { "Come closer" }
            }
            Tile { label: "TiltCard",
                TiltCard { "Tilt" }
            }
        }

        Section { title: "Cursor", note: "MouseFollow and CursorTrail are mounted page-wide, above.",
            Tile { label: "Spotlight",
                Spotlight { "Move the pointer across this panel." }
            }
        }

        Section { title: "Card grid", note: "A highlight slides in behind the hovered card.",
            div { class: "wide",
                CardHover {
                    items: vec![
                        CardHoverItem::new("Compositor-driven", "Every animation is CSS keyframes."),
                        CardHoverItem::new("One dependency", "Just dioxus. No motion library."),
                        CardHoverItem::new("Nothing to set up", "Components inject their own CSS."),
                    ],
                }
            }
        }

        Section { title: "Buttons", note: "Twelve interactions behind one component. Bring your own icons.",
            for (label , interaction) in BUTTONS {
                Tile { label,
                    AnimatedButton {
                        label: "Action",
                        alt_label: "Done",
                        interaction,
                        alt_color: "#34d399",
                        icon: rsx! {
                            Chevron {}
                        },
                        alt_icon: rsx! {
                            Check {}
                        },
                    }
                }
            }
            Tile { label: "FocusBlurLinks",
                FocusBlurLinks {
                    items: vec![
                        ("@X".into(), "#".into()),
                        ("@Threads".into(), "#".into()),
                        ("@GitHub".into(), "#".into()),
                    ],
                }
            }
        }

        Section { title: "Card spreads", note: "Hover a stack to fan it out.",
            for (label , layout) in SPREADS {
                Tile { label,
                    CardSpread { layout, card_color: "#3f3f46" }
                }
            }
        }

        Section { title: "Carousels", note: "Click a slide or a dot to move the active index.",
            div { class: "wide",
                CardCarousel { items: photos() }
            }
            div { class: "wide",
                CardCoverFlow { items: photos() }
            }
            div { class: "wide",
                CardTimeMachine { items: photos() }
            }
        }

        Section { title: "Scroll", note: "The bar at the top of the page fills as you scroll.",
            div { class: "wide",
                ScrollReveal {
                    p { "This block waited until it entered the viewport." }
                }
                StickyReveal {
                    items: vec![
                        StickyRevealItem::new("Read on", "The panel swaps as each block scrolls past.")
                            .with_visual("First"),
                        StickyRevealItem::new("Keep going", "Driven by an IntersectionObserver, not a scroll listener.")
                            .with_visual("Second"),
                        StickyRevealItem::new("Last one", "No scroll maths in Rust at all.")
                            .with_visual("Third"),
                    ],
                }
            }
        }

        footer { "MIT or Apache-2.0. Ported from Amicro." }
    }
}

const BUTTONS: [(&str, ButtonInteraction); 12] = [
    ("SlideArrow", ButtonInteraction::SlideArrow),
    ("Sparkle", ButtonInteraction::Sparkle),
    ("Morph", ButtonInteraction::Morph),
    ("ColorMorph", ButtonInteraction::ColorMorph),
    ("Pulse", ButtonInteraction::Pulse),
    ("Rotate", ButtonInteraction::Rotate),
    ("Shake", ButtonInteraction::Shake),
    ("Ring", ButtonInteraction::Ring),
    ("Glare", ButtonInteraction::Glare),
    ("TextReveal", ButtonInteraction::TextReveal),
    ("Magnetic", ButtonInteraction::Magnetic),
    ("ExpandRing", ButtonInteraction::ExpandRing),
];

const SPREADS: [(&str, CardSpreadLayout); 9] = [
    ("Arc5", CardSpreadLayout::Arc5),
    ("Arc7", CardSpreadLayout::Arc7),
    ("LongArc5", CardSpreadLayout::LongArc5),
    ("LinearSpread", CardSpreadLayout::LinearSpread),
    ("CornerFan", CardSpreadLayout::CornerFan),
    ("StampArc", CardSpreadLayout::StampArc),
    ("CascadeStagger", CardSpreadLayout::CascadeStagger),
    ("ScatterSpread", CardSpreadLayout::ScatterSpread),
    ("WheelFan", CardSpreadLayout::WheelFan),
];

/// Inline SVG placeholders so the gallery pulls in no icon crate.
#[component]
fn Chevron() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
            path { d: "M5 12h14M13 6l6 6-6 6" }
        }
    }
}

#[component]
fn Check() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
            path { d: "M4 12l5 5L20 6" }
        }
    }
}

/// A plain block for the entrance demos to move around.
#[component]
fn Swatch() -> Element {
    rsx! {
        div { class: "box" }
    }
}

/// Locally drawn gradients rather than remote photos, so the gallery works
/// offline.
fn photos() -> Vec<CardItem> {
    const GRADIENTS: [(&str, &str, &str); 5] = [
        ("#f97316", "#ec4899", "Sunset"),
        ("#0ea5e9", "#6366f1", "Dusk"),
        ("#22c55e", "#14b8a6", "Forest"),
        ("#eab308", "#f97316", "Sunlight"),
        ("#8b5cf6", "#0ea5e9", "Hills"),
    ];
    const DATES: [&str; 5] = ["Today", "1d ago", "1w ago", "1m ago", "1y ago"];
    GRADIENTS
        .iter()
        .zip(DATES)
        .map(|((from, to, title), date)| {
            let svg = format!(
                "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 3 4'>\
                 <defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'>\
                 <stop offset='0' stop-color='{from}'/><stop offset='1' stop-color='{to}'/>\
                 </linearGradient></defs><rect width='3' height='4' fill='url(%23g)'/></svg>"
            );
            CardItem::new(format!("data:image/svg+xml,{svg}"), *title).with_date(date)
        })
        .collect()
}

const GALLERY_CSS: &str = r#"
:root{color-scheme:dark;--fg:#e4e4e7;--dim:#71717a;--line:rgba(255,255,255,.08)}
body{margin:0;padding:0 24px 96px;background:#09090b;color:var(--fg);font-family:ui-sans-serif,system-ui,-apple-system,sans-serif}
header{padding:96px 0 48px;text-align:center}
header h1{margin:0;font-size:2.5rem;letter-spacing:-.03em}
header p{margin:12px 0 0;color:var(--dim)}
section{max-width:1120px;margin:0 auto 72px}
section h2{margin:0;font-size:1.25rem;letter-spacing:-.02em}
.note{margin:6px 0 24px;color:var(--dim);font-size:.875rem}
.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:12px}
.tile{display:flex;flex-direction:column;gap:8px;margin:0;padding:16px 8px;border:1px solid var(--line);border-radius:12px}
.stage{display:flex;align-items:center;justify-content:center;min-height:96px;overflow:hidden}
figcaption{color:var(--dim);font-size:11px;text-align:center;overflow:hidden;text-overflow:ellipsis}
.wide{grid-column:1/-1;margin-top:16px;padding:24px;border:1px solid var(--line);border-radius:12px}
.box{width:64px;height:64px;border-radius:12px;background:linear-gradient(135deg,#6366f1,#0ea5e9)}
footer{padding:48px 0;color:var(--dim);font-size:.8125rem;text-align:center}
"#;
