//! Loading indicators: spinners, progress rings, skeletons and activity
//! animations.
//!
//! Every loader takes the same core props — `size` in pixels, `color` as any
//! CSS colour (defaulting to `currentColor` so it inherits from its parent),
//! `duration` in seconds, plus `class` and arbitrary passthrough attributes.
//! Each renders a `role="status"` root so screen readers announce it.
//!
//! ```rust, no_run
//! # use dioxus::prelude::*;
//! use dioxus_fx::loading::IosSpinner;
//!
//! fn Busy() -> Element {
//!     rsx! { IosSpinner { size: 24.0, color: "#3b82f6" } }
//! }
//! ```

mod accordion_loader;
mod app_icon_load;
mod apple_breathe;
mod apple_equalizer;
mod apple_icon_morph;
mod apple_pulse_dots;
mod apple_scale_pulse;
mod apple_sound_wave;
mod apple_text_reveal;
mod apple_unlock;
mod arc_tracer;
mod bar_cascade;
mod bar_sweep;
mod bobbing_dots;
mod bounce_dots;
mod bouncing_bars;
mod bouncing_dots;
mod bouncing_lines;
mod bouncing_square;
mod breathe_ring;
mod breathing_glow;
mod breathing_square;
mod circular_bars;
mod classic_spinner;
mod clock_spinner;
mod comet_spinner;
mod concentric_pulse;
mod concentric_ring;
mod concentric_squares;
mod conveyor_loop;
mod cross_spinner;
mod cube_flip_spring;
mod dash_ring;
mod dashed_spiral;
mod diamond_grid;
mod diamond_rotate_spring;
mod dot_spinner;
mod dots_ring;
mod double_ring;
mod drop_dot;
mod dual_arc;
mod dynamic_island;
mod elastic_bars;
mod elastic_square;
mod expanding_cross;
mod face_id_scan;
mod fade_arc;
mod fade_dots;
mod flip_square;
mod floating_diamonds;
mod fluid_bars;
mod fluid_diamond;
mod fluid_dot_orbit;
mod fluid_skeleton;
mod gears;
mod glassmorphic_card;
mod gradient_arc;
mod grid_dots;
mod haptic_ring;
mod heartbeat;
mod hexagon_spinner;
mod hourglass;
mod infinity_path;
mod intersecting_rings;
mod ios_spinner;
mod line_spinner;
mod liquid_dots;
mod mac_terminal;
mod magnetic_dots;
mod minimal_triangle;
mod morph_dot_ring;
mod morph_loader;
mod morphing_bars;
mod morphing_infinity;
mod morphing_ring;
mod morphing_shape;
mod newtons_cradle;
mod offset_rings;
mod orbiting_circles;
mod orbiting_dot;
mod origami_shape;
mod pendulum;
mod pulsating_dots;
mod pulse;
mod pulse_dot;
mod pulse_dots;
mod pulse_square;
mod pumping_heart;
mod radar_sweep;
mod ring_sweep;
mod ripple_effect;
mod rotating_cross;
mod rotating_triangle;
mod shape_shift_grid;
mod shimmer_line;
mod siri_wave;
mod skeleton;
mod skeleton_loader;
mod sliding_bars;
mod smooth_dot_shift;
mod smooth_ring;
mod smooth_rounded_square;
mod spinning_squares;
mod spiral_spinner;
mod spring_bars;
mod spring_dot_matrix;
mod spring_hexagon;
mod spring_ring_expand;
mod spring_text_pop;
mod square_accordion;
mod square_grid;
mod square_snake;
mod square_spinner;
mod stacked_bar_pulse;
mod swapping_dots;
mod swirling_spinner;
mod symmetric_wave;
mod terminal_loader;
mod text_blink;
mod text_dots;
mod text_morph;
mod text_shimmer;
mod text_shimmer_wave;
mod trailing_dots;
mod triple_dot_spinner;
mod twin_orbit;
mod typing;
mod typing_indicator;
mod wandering_cube;
mod watch_spinner;
mod wave_dots;
mod wave_physics_loader;
mod waveform_loader;
mod zig_zag_pulse;

pub use accordion_loader::{AccordionLoader, AccordionLoaderProps};
pub use app_icon_load::{AppIconLoad, AppIconLoadProps};
pub use apple_breathe::{AppleBreathe, AppleBreatheProps};
pub use apple_equalizer::{AppleEqualizer, AppleEqualizerProps};
pub use apple_icon_morph::{AppleIconMorph, AppleIconMorphProps};
pub use apple_pulse_dots::{ApplePulseDots, ApplePulseDotsProps};
pub use apple_scale_pulse::{AppleScalePulse, AppleScalePulseProps};
pub use apple_sound_wave::{AppleSoundWave, AppleSoundWaveProps};
pub use apple_text_reveal::{AppleTextReveal, AppleTextRevealProps};
pub use apple_unlock::{AppleUnlock, AppleUnlockProps};
pub use arc_tracer::{ArcTracer, ArcTracerProps};
pub use bar_cascade::{BarCascade, BarCascadeProps};
pub use bar_sweep::{BarSweep, BarSweepProps};
pub use bobbing_dots::{BobbingDots, BobbingDotsProps};
pub use bounce_dots::{BounceDots, BounceDotsProps};
pub use bouncing_bars::{BouncingBars, BouncingBarsProps};
pub use bouncing_dots::{BouncingDots, BouncingDotsProps};
pub use bouncing_lines::{BouncingLines, BouncingLinesProps};
pub use bouncing_square::{BouncingSquare, BouncingSquareProps};
pub use breathe_ring::{BreatheRing, BreatheRingProps};
pub use breathing_glow::{BreathingGlow, BreathingGlowProps};
pub use breathing_square::{BreathingSquare, BreathingSquareProps};
pub use circular_bars::{CircularBars, CircularBarsProps};
pub use classic_spinner::{ClassicSpinner, ClassicSpinnerProps};
pub use clock_spinner::{ClockSpinner, ClockSpinnerProps};
pub use comet_spinner::{CometSpinner, CometSpinnerProps};
pub use concentric_pulse::{ConcentricPulse, ConcentricPulseProps};
pub use concentric_ring::{ConcentricRing, ConcentricRingProps};
pub use concentric_squares::{ConcentricSquares, ConcentricSquaresProps};
pub use conveyor_loop::{ConveyorLoop, ConveyorLoopProps};
pub use cross_spinner::{CrossSpinner, CrossSpinnerProps};
pub use cube_flip_spring::{CubeFlipSpring, CubeFlipSpringProps};
pub use dash_ring::{DashRing, DashRingProps};
pub use dashed_spiral::{DashedSpiral, DashedSpiralProps};
pub use diamond_grid::{DiamondGrid, DiamondGridProps};
pub use diamond_rotate_spring::{DiamondRotateSpring, DiamondRotateSpringProps};
pub use dot_spinner::{DotSpinner, DotSpinnerProps};
pub use dots_ring::{DotsRing, DotsRingProps};
pub use double_ring::{DoubleRing, DoubleRingProps};
pub use drop_dot::{DropDot, DropDotProps};
pub use dual_arc::{DualArc, DualArcProps};
pub use dynamic_island::{DynamicIsland, DynamicIslandProps};
pub use elastic_bars::{ElasticBars, ElasticBarsProps};
pub use elastic_square::{ElasticSquare, ElasticSquareProps};
pub use expanding_cross::{ExpandingCross, ExpandingCrossProps};
pub use face_id_scan::{FaceIdScan, FaceIdScanProps};
pub use fade_arc::{FadeArc, FadeArcProps};
pub use fade_dots::{FadeDots, FadeDotsProps};
pub use flip_square::{FlipSquare, FlipSquareProps};
pub use floating_diamonds::{FloatingDiamonds, FloatingDiamondsProps};
pub use fluid_bars::{FluidBars, FluidBarsProps};
pub use fluid_diamond::{FluidDiamond, FluidDiamondProps};
pub use fluid_dot_orbit::{FluidDotOrbit, FluidDotOrbitProps};
pub use fluid_skeleton::{FluidSkeleton, FluidSkeletonProps};
pub use gears::{Gears, GearsProps};
pub use glassmorphic_card::{GlassmorphicCard, GlassmorphicCardProps};
pub use gradient_arc::{GradientArc, GradientArcProps};
pub use grid_dots::{GridDots, GridDotsProps};
pub use haptic_ring::{HapticRing, HapticRingProps};
pub use heartbeat::{Heartbeat, HeartbeatProps};
pub use hexagon_spinner::{HexagonSpinner, HexagonSpinnerProps};
pub use hourglass::{Hourglass, HourglassProps};
pub use infinity_path::{InfinityPath, InfinityPathProps};
pub use intersecting_rings::{IntersectingRings, IntersectingRingsProps};
pub use ios_spinner::{IosSpinner, IosSpinnerProps};
pub use line_spinner::{LineSpinner, LineSpinnerProps};
pub use liquid_dots::{LiquidDots, LiquidDotsProps};
pub use mac_terminal::{MacTerminal, MacTerminalProps};
pub use magnetic_dots::{MagneticDots, MagneticDotsProps};
pub use minimal_triangle::{MinimalTriangle, MinimalTriangleProps};
pub use morph_dot_ring::{MorphDotRing, MorphDotRingProps};
pub use morph_loader::{MorphLoader, MorphLoaderProps};
pub use morphing_bars::{MorphingBars, MorphingBarsProps};
pub use morphing_infinity::{MorphingInfinity, MorphingInfinityProps};
pub use morphing_ring::{MorphingRing, MorphingRingProps};
pub use morphing_shape::{MorphingShape, MorphingShapeProps};
pub use newtons_cradle::{NewtonsCradle, NewtonsCradleProps};
pub use offset_rings::{OffsetRings, OffsetRingsProps};
pub use orbiting_circles::{OrbitingCircles, OrbitingCirclesProps};
pub use orbiting_dot::{OrbitingDot, OrbitingDotProps};
pub use origami_shape::{OrigamiShape, OrigamiShapeProps};
pub use pendulum::{Pendulum, PendulumProps};
pub use pulsating_dots::{PulsatingDots, PulsatingDotsProps};
pub use pulse::{Pulse, PulseProps};
pub use pulse_dot::{PulseDot, PulseDotProps};
pub use pulse_dots::{PulseDots, PulseDotsProps};
pub use pulse_square::{PulseSquare, PulseSquareProps};
pub use pumping_heart::{PumpingHeart, PumpingHeartProps};
pub use radar_sweep::{RadarSweep, RadarSweepProps};
pub use ring_sweep::{RingSweep, RingSweepProps};
pub use ripple_effect::{RippleEffect, RippleEffectProps};
pub use rotating_cross::{RotatingCross, RotatingCrossProps};
pub use rotating_triangle::{RotatingTriangle, RotatingTriangleProps};
pub use shape_shift_grid::{ShapeShiftGrid, ShapeShiftGridProps};
pub use shimmer_line::{ShimmerLine, ShimmerLineProps};
pub use siri_wave::{SiriWave, SiriWaveProps};
pub use skeleton::{Skeleton, SkeletonProps};
pub use skeleton_loader::{SkeletonLoader, SkeletonLoaderProps};
pub use sliding_bars::{SlidingBars, SlidingBarsProps};
pub use smooth_dot_shift::{SmoothDotShift, SmoothDotShiftProps};
pub use smooth_ring::{SmoothRing, SmoothRingProps};
pub use smooth_rounded_square::{SmoothRoundedSquare, SmoothRoundedSquareProps};
pub use spinning_squares::{SpinningSquares, SpinningSquaresProps};
pub use spiral_spinner::{SpiralSpinner, SpiralSpinnerProps};
pub use spring_bars::{SpringBars, SpringBarsProps};
pub use spring_dot_matrix::{SpringDotMatrix, SpringDotMatrixProps};
pub use spring_hexagon::{SpringHexagon, SpringHexagonProps};
pub use spring_ring_expand::{SpringRingExpand, SpringRingExpandProps};
pub use spring_text_pop::{SpringTextPop, SpringTextPopProps};
pub use square_accordion::{SquareAccordion, SquareAccordionProps};
pub use square_grid::{SquareGrid, SquareGridProps};
pub use square_snake::{SquareSnake, SquareSnakeProps};
pub use square_spinner::{SquareSpinner, SquareSpinnerProps};
pub use stacked_bar_pulse::{StackedBarPulse, StackedBarPulseProps};
pub use swapping_dots::{SwappingDots, SwappingDotsProps};
pub use swirling_spinner::{SwirlingSpinner, SwirlingSpinnerProps};
pub use symmetric_wave::{SymmetricWave, SymmetricWaveProps};
pub use terminal_loader::{TerminalLoader, TerminalLoaderProps};
pub use text_blink::{TextBlink, TextBlinkProps};
pub use text_dots::{TextDots, TextDotsProps};
pub use text_morph::{TextMorph, TextMorphProps};
pub use text_shimmer::{TextShimmer, TextShimmerProps};
pub use text_shimmer_wave::{TextShimmerWave, TextShimmerWaveProps};
pub use trailing_dots::{TrailingDots, TrailingDotsProps};
pub use triple_dot_spinner::{TripleDotSpinner, TripleDotSpinnerProps};
pub use twin_orbit::{TwinOrbit, TwinOrbitProps};
pub use typing::{Typing, TypingProps};
pub use typing_indicator::{TypingIndicator, TypingIndicatorProps};
pub use wandering_cube::{WanderingCube, WanderingCubeProps};
pub use watch_spinner::{WatchSpinner, WatchSpinnerProps};
pub use wave_dots::{WaveDots, WaveDotsProps};
pub use wave_physics_loader::{WavePhysicsLoader, WavePhysicsLoaderProps};
pub use waveform_loader::{WaveformLoader, WaveformLoaderProps};
pub use zig_zag_pulse::{ZigZagPulse, ZigZagPulseProps};

pub(crate) const CSS: &[&str] = &[
    accordion_loader::CSS,
    app_icon_load::CSS,
    apple_breathe::CSS,
    apple_equalizer::CSS,
    apple_icon_morph::CSS,
    apple_pulse_dots::CSS,
    apple_scale_pulse::CSS,
    apple_sound_wave::CSS,
    apple_text_reveal::CSS,
    apple_unlock::CSS,
    arc_tracer::CSS,
    bar_cascade::CSS,
    bar_sweep::CSS,
    bobbing_dots::CSS,
    bounce_dots::CSS,
    bouncing_bars::CSS,
    bouncing_dots::CSS,
    bouncing_lines::CSS,
    bouncing_square::CSS,
    breathe_ring::CSS,
    breathing_glow::CSS,
    breathing_square::CSS,
    circular_bars::CSS,
    classic_spinner::CSS,
    clock_spinner::CSS,
    comet_spinner::CSS,
    concentric_pulse::CSS,
    concentric_ring::CSS,
    concentric_squares::CSS,
    conveyor_loop::CSS,
    cross_spinner::CSS,
    cube_flip_spring::CSS,
    dash_ring::CSS,
    dashed_spiral::CSS,
    diamond_grid::CSS,
    diamond_rotate_spring::CSS,
    dot_spinner::CSS,
    dots_ring::CSS,
    double_ring::CSS,
    drop_dot::CSS,
    dual_arc::CSS,
    dynamic_island::CSS,
    elastic_bars::CSS,
    elastic_square::CSS,
    expanding_cross::CSS,
    face_id_scan::CSS,
    fade_arc::CSS,
    fade_dots::CSS,
    flip_square::CSS,
    floating_diamonds::CSS,
    fluid_bars::CSS,
    fluid_diamond::CSS,
    fluid_dot_orbit::CSS,
    fluid_skeleton::CSS,
    gears::CSS,
    glassmorphic_card::CSS,
    gradient_arc::CSS,
    grid_dots::CSS,
    haptic_ring::CSS,
    heartbeat::CSS,
    hexagon_spinner::CSS,
    hourglass::CSS,
    infinity_path::CSS,
    intersecting_rings::CSS,
    ios_spinner::CSS,
    line_spinner::CSS,
    liquid_dots::CSS,
    mac_terminal::CSS,
    magnetic_dots::CSS,
    minimal_triangle::CSS,
    morph_dot_ring::CSS,
    morph_loader::CSS,
    morphing_bars::CSS,
    morphing_infinity::CSS,
    morphing_ring::CSS,
    morphing_shape::CSS,
    newtons_cradle::CSS,
    offset_rings::CSS,
    orbiting_circles::CSS,
    orbiting_dot::CSS,
    origami_shape::CSS,
    pendulum::CSS,
    pulsating_dots::CSS,
    pulse::CSS,
    pulse_dot::CSS,
    pulse_dots::CSS,
    pulse_square::CSS,
    pumping_heart::CSS,
    radar_sweep::CSS,
    ring_sweep::CSS,
    ripple_effect::CSS,
    rotating_cross::CSS,
    rotating_triangle::CSS,
    shape_shift_grid::CSS,
    shimmer_line::CSS,
    siri_wave::CSS,
    skeleton::CSS,
    skeleton_loader::CSS,
    sliding_bars::CSS,
    smooth_dot_shift::CSS,
    smooth_ring::CSS,
    smooth_rounded_square::CSS,
    spinning_squares::CSS,
    spiral_spinner::CSS,
    spring_bars::CSS,
    spring_dot_matrix::CSS,
    spring_hexagon::CSS,
    spring_ring_expand::CSS,
    spring_text_pop::CSS,
    square_accordion::CSS,
    square_grid::CSS,
    square_snake::CSS,
    square_spinner::CSS,
    stacked_bar_pulse::CSS,
    swapping_dots::CSS,
    swirling_spinner::CSS,
    symmetric_wave::CSS,
    terminal_loader::CSS,
    text_blink::CSS,
    text_dots::CSS,
    text_morph::CSS,
    text_shimmer::CSS,
    text_shimmer_wave::CSS,
    trailing_dots::CSS,
    triple_dot_spinner::CSS,
    twin_orbit::CSS,
    typing::CSS,
    typing_indicator::CSS,
    wandering_cube::CSS,
    watch_spinner::CSS,
    wave_dots::CSS,
    wave_physics_loader::CSS,
    waveform_loader::CSS,
    zig_zag_pulse::CSS,
];

pub(crate) use wave_physics_loader::keyframes as wave_physics_keyframes;
