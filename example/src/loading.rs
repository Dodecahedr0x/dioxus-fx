//! Stories for every loader in `dioxus_fx::loading`.
//!
//! Each story exposes the loader's own props as showcase controls; leaving a
//! control untouched renders the component's documented default.

use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_fx::loading::*;
use dioxus_showcase::prelude::*;

/// Four stacked bars that collapse and expand from the left in sequence.
#[story(title = "Loading/AccordionLoader", tags = ["loading"])]
pub fn accordion_loader(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        AccordionLoader {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A rounded app tile with a progress ring sweeping inside it.
#[story(title = "Loading/AppIconLoad", tags = ["loading"])]
pub fn app_icon_load(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        AppIconLoad {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Six translucent dots that drift outward and swell, then draw back in.
#[story(title = "Loading/AppleBreathe", tags = ["loading"])]
pub fn apple_breathe(size: f64, color: String, accent_color: String, duration: f64) -> Element {
    rsx! {
        AppleBreathe {
            size: num(size, 48.0),
            color: txt(color, "#2dd4bf"),
            accent_color: txt(accent_color, "#22d3ee"),
            duration: num(duration, 3.6),
        }
    }
}

/// Four audio-meter bars jumping at offset intervals.
#[story(title = "Loading/AppleEqualizer", tags = ["loading"])]
pub fn apple_equalizer(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        AppleEqualizer {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.8),
        }
    }
}

/// A squircle that rounds itself into a circle and back while rotating.
#[story(title = "Loading/AppleIconMorph", tags = ["loading"])]
pub fn apple_icon_morph(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        AppleIconMorph {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Three dots that swell and fade in a rolling sequence.
#[story(title = "Loading/ApplePulseDots", tags = ["loading"])]
pub fn apple_pulse_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ApplePulseDots {
            size: num(size, 10.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// A solid centre with two rings expanding out of it and fading.
#[story(title = "Loading/AppleScalePulse", tags = ["loading"])]
pub fn apple_scale_pulse(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        AppleScalePulse {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Five bars forming a symmetric wave that swells from the centre outward.
#[story(title = "Loading/AppleSoundWave", tags = ["loading"])]
pub fn apple_sound_wave(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        AppleSoundWave {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A word that scrolls up through a fixed-height window, over and over.
#[story(title = "Loading/AppleTextReveal", tags = ["loading"])]
pub fn apple_text_reveal(text: String, color: String, duration: f64) -> Element {
    rsx! {
        AppleTextReveal {
            text: txt(text, "Loading"),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Dimmed text with a bright highlight sweeping across it, iOS lock-screen style.
#[story(title = "Loading/AppleUnlock", tags = ["loading"])]
pub fn apple_unlock(text: String, color: String, duration: f64) -> Element {
    rsx! {
        AppleUnlock {
            text: txt(text, "Slide to unlock"),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.2),
        }
    }
}

/// An arc that draws itself around a track, then unwinds off the other end.
#[story(title = "Loading/ArcTracer", tags = ["loading"])]
pub fn arc_tracer(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ArcTracer {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Five bars rising and falling in a cascading run.
#[story(title = "Loading/BarCascade", tags = ["loading"])]
pub fn bar_cascade(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BarCascade {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A bead sliding from end to end inside a pill-shaped track.
#[story(title = "Loading/BarSweep", tags = ["loading"])]
pub fn bar_sweep(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BarSweep {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Three dots bobbing up and down in a rolling wave.
#[story(title = "Loading/BobbingDots", tags = ["loading"])]
pub fn bobbing_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BobbingDots {
            size: num(size, 12.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// Three small dots hopping in quick succession.
#[story(title = "Loading/BounceDots", tags = ["loading"])]
pub fn bounce_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BounceDots {
            size: num(size, 10.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.6),
        }
    }
}

/// Three bars squeezing and releasing along their length.
#[story(title = "Loading/BouncingBars", tags = ["loading"])]
pub fn bouncing_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BouncingBars {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Three dots bouncing off a baseline, stretching as they leave it.
#[story(title = "Loading/BouncingDots", tags = ["loading"])]
pub fn bouncing_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BouncingDots {
            size: num(size, 64.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.8),
        }
    }
}

/// Three stacked lines extending from the left in sequence.
#[story(title = "Loading/BouncingLines", tags = ["loading"])]
pub fn bouncing_lines(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BouncingLines {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A square bouncing on its shadow, squashing on impact.
#[story(title = "Loading/BouncingSquare", tags = ["loading"])]
pub fn bouncing_square(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BouncingSquare {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.6),
        }
    }
}

/// A ring that swells as its stroke thins, then draws back in.
#[story(title = "Loading/BreatheRing", tags = ["loading"])]
pub fn breathe_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BreatheRing {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 3.0),
        }
    }
}

/// A bright core sitting inside a halo that breathes in and out.
#[story(title = "Loading/BreathingGlow", tags = ["loading"])]
pub fn breathing_glow(size: f64, color: String, glow_color: String, duration: f64) -> Element {
    rsx! {
        BreathingGlow {
            size: num(size, 48.0),
            color: txt(color, "#ffffff"),
            glow_color: txt(glow_color, "#3b82f6"),
            duration: num(duration, 2.0),
        }
    }
}

/// A solid square inflating into a disc and back as it turns.
#[story(title = "Loading/BreathingSquare", tags = ["loading"])]
pub fn breathing_square(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        BreathingSquare {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Eight radial spokes that stretch and shrink around the dial in sequence.
#[story(title = "Loading/CircularBars", tags = ["loading"])]
pub fn circular_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        CircularBars {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// The twelve-spoke system spinner, each spoke fading in turn.
#[story(title = "Loading/ClassicSpinner", tags = ["loading"])]
pub fn classic_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ClassicSpinner {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A clock face whose two hands sweep at a 6:1 ratio.
#[story(title = "Loading/ClockSpinner", tags = ["loading"])]
pub fn clock_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ClockSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A ring with a comet tail that fades from solid to nothing as it spins.
#[story(title = "Loading/CometSpinner", tags = ["loading"])]
pub fn comet_spinner(size: f64, color: String, thickness: f64, duration: f64) -> Element {
    rsx! {
        CometSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            thickness: num(thickness, 4.0),
            duration: num(duration, 1.0),
        }
    }
}

/// Three hairline rings expanding out of a single point, like sonar.
#[story(title = "Loading/ConcentricPulse", tags = ["loading"])]
pub fn concentric_pulse(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ConcentricPulse {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Three nested rings turning at different speeds, the middle one reversed.
#[story(title = "Loading/ConcentricRing", tags = ["loading"])]
pub fn concentric_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ConcentricRing {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two nested squares turning against each other, one growing as the other shrinks.
#[story(title = "Loading/ConcentricSquares", tags = ["loading"])]
pub fn concentric_squares(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ConcentricSquares {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A track of beads sliding endlessly to the left inside a pill.
#[story(title = "Loading/ConveyorLoop", tags = ["loading"])]
pub fn conveyor_loop(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ConveyorLoop {
            size: num(size, 64.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A plus sign that steps around a full turn in quarters.
#[story(title = "Loading/CrossSpinner", tags = ["loading"])]
pub fn cross_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        CrossSpinner {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A rounded tile flipping over one axis and then the other, with a little bounce.
#[story(title = "Loading/CubeFlipSpring", tags = ["loading"])]
pub fn cube_flip_spring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        CubeFlipSpring {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A dashed circle rotating slowly, like a selection marquee.
#[story(title = "Loading/DashRing", tags = ["loading"])]
pub fn dash_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DashRing {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 4.0),
        }
    }
}

/// A dashed ring turning while it breathes in and out.
#[story(title = "Loading/DashedSpiral", tags = ["loading"])]
pub fn dashed_spiral(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DashedSpiral {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 4.0),
        }
    }
}

/// Four tiles arranged as a diamond, blinking in a rolling sequence.
#[story(title = "Loading/DiamondGrid", tags = ["loading"])]
pub fn diamond_grid(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DiamondGrid {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A diamond that snaps a quarter-turn at a time, overshooting each stop.
#[story(title = "Loading/DiamondRotateSpring", tags = ["loading"])]
pub fn diamond_rotate_spring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DiamondRotateSpring {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Eight dots on a ring, each dimming a beat after the last.
#[story(title = "Loading/DotSpinner", tags = ["loading"])]
pub fn dot_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DotSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Eight dots on a ring that shrink and fade in a travelling wave.
#[story(title = "Loading/DotsRing", tags = ["loading"])]
pub fn dots_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DotsRing {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Two rings chasing each other in opposite directions.
#[story(title = "Loading/DoubleRing", tags = ["loading"])]
pub fn double_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DoubleRing {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A droplet that falls onto a soft shadow, squashing as it lands.
#[story(title = "Loading/DropDot", tags = ["loading"])]
pub fn drop_dot(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DropDot {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Two opposing arcs spinning while the ring pumps in and out.
#[story(title = "Loading/DualArc", tags = ["loading"])]
pub fn dual_arc(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        DualArc {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// The iOS pill that widens and narrows with a live-activity indicator inside.
#[story(title = "Loading/DynamicIsland", tags = ["loading"])]
pub fn dynamic_island(color: String, accent_color: String, duration: f64) -> Element {
    rsx! {
        DynamicIsland {
            color: txt(color, "#09090b"),
            accent_color: txt(accent_color, "#10b981"),
            duration: num(duration, 2.2),
        }
    }
}

/// Three bars stretching tall and snapping back, one after another.
#[story(title = "Loading/ElasticBars", tags = ["loading"])]
pub fn elastic_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ElasticBars {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.1),
        }
    }
}

/// A squishy square bouncing off a baseline.
#[story(title = "Loading/ElasticSquare", tags = ["loading"])]
pub fn elastic_square(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ElasticSquare {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.8),
        }
    }
}

/// A cross whose arms take turns stretching to full width and height.
#[story(title = "Loading/ExpandingCross", tags = ["loading"])]
pub fn expanding_cross(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ExpandingCross {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A framed face outline with a scan line sweeping down and back.
#[story(title = "Loading/FaceIdScan", tags = ["loading"])]
pub fn face_id_scan(size: f64, color: String, scan_color: String, duration: f64) -> Element {
    rsx! {
        FaceIdScan {
            size: num(size, 48.0),
            color: txt(color, "#22c55e"),
            scan_color: txt(scan_color, "rgba(74,222,128,.3)"),
            duration: num(duration, 2.5),
        }
    }
}

/// A fixed arc riding a faint track, spinning at a constant rate.
#[story(title = "Loading/FadeArc", tags = ["loading"])]
pub fn fade_arc(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FadeArc {
            size: num(size, 36.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Four dots fading up and out in a marching sequence.
#[story(title = "Loading/FadeDots", tags = ["loading"])]
pub fn fade_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FadeDots {
            size: num(size, 10.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A tile flipping through all four faces of a two-axis cycle.
#[story(title = "Loading/FlipSquare", tags = ["loading"])]
pub fn flip_square(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FlipSquare {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Three diamonds bobbing up and down in a rolling wave.
#[story(title = "Loading/FloatingDiamonds", tags = ["loading"])]
pub fn floating_diamonds(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FloatingDiamonds {
            size: num(size, 12.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Four bars swaying like reeds, each a beat behind the last.
#[story(title = "Loading/FluidBars", tags = ["loading"])]
pub fn fluid_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FluidBars {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A diamond flattening and rebounding, as if made of liquid.
#[story(title = "Loading/FluidDiamond", tags = ["loading"])]
pub fn fluid_diamond(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FluidDiamond {
            size: num(size, 24.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A still centre dot with a smaller satellite circling it.
#[story(title = "Loading/FluidDotOrbit", tags = ["loading"])]
pub fn fluid_dot_orbit(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        FluidDotOrbit {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A rounded placeholder block with a highlight sweeping across it.
#[story(title = "Loading/FluidSkeleton", tags = ["loading"])]
pub fn fluid_skeleton(
    width: String,
    height: String,
    radius: String,
    color: String,
    duration: f64,
) -> Element {
    rsx! {
        FluidSkeleton {
            width: txt(width, "96px"),
            height: txt(height, "40px"),
            radius: txt(radius, "12px"),
            color: txt(color, "rgba(255,255,255,.6)"),
            duration: num(duration, 1.5),
        }
    }
}

/// Two interlocking dashed cogs turning against each other.
#[story(title = "Loading/Gears", tags = ["loading"])]
pub fn gears(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        Gears {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 4.0),
        }
    }
}

/// A frosted glass panel with a drifting glow behind a small spinner.
#[story(title = "Loading/GlassmorphicCard", tags = ["loading"])]
pub fn glassmorphic_card(
    width: String,
    height: String,
    color: String,
    glow_color: String,
    duration: f64,
) -> Element {
    rsx! {
        GlassmorphicCard {
            width: txt(width, "80px"),
            height: txt(height, "64px"),
            color: txt(color, "currentColor"),
            glow_color: txt(glow_color, "rgba(161,161,170,.65)"),
            duration: num(duration, 3.0),
        }
    }
}

/// A gradient-filled ring spinning at a constant rate.
#[story(title = "Loading/GradientArc", tags = ["loading"])]
pub fn gradient_arc(size: f64, from_color: String, to_color: String, duration: f64) -> Element {
    rsx! {
        GradientArc {
            size: num(size, 40.0),
            from_color: txt(from_color, "#00f2fe"),
            to_color: txt(to_color, "#4facfe"),
            duration: num(duration, 1.2),
        }
    }
}

/// A three-by-three field of dots pulsing on a diagonal wave.
#[story(title = "Loading/GridDots", tags = ["loading"])]
pub fn grid_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        GridDots {
            size: num(size, 10.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A ring that snaps through quarter turns with a springy overshoot.
#[story(title = "Loading/HapticRing", tags = ["loading"])]
pub fn haptic_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        HapticRing {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// An ECG trace scrolling across the screen.
#[story(title = "Loading/Heartbeat", tags = ["loading"])]
pub fn heartbeat(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        Heartbeat {
            size: num(size, 64.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A hexagon outline being traced by a travelling stroke.
#[story(title = "Loading/HexagonSpinner", tags = ["loading"])]
pub fn hexagon_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        HexagonSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two stacked triangles flipping end over end, like sand running out.
#[story(title = "Loading/Hourglass", tags = ["loading"])]
pub fn hourglass(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        Hourglass {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 3.0),
        }
    }
}

/// A stroke chasing itself around a lemniscate.
#[story(title = "Loading/InfinityPath", tags = ["loading"])]
pub fn infinity_path(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        InfinityPath {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two rings tumbling through each other in three dimensions.
#[story(title = "Loading/IntersectingRings", tags = ["loading"])]
pub fn intersecting_rings(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        IntersectingRings {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.2),
        }
    }
}

/// The iOS activity indicator: twelve tapered spokes fading around the dial.
#[story(title = "Loading/IosSpinner", tags = ["loading"])]
pub fn ios_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        IosSpinner {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A single needle sweeping inside a hairline circle.
#[story(title = "Loading/LineSpinner", tags = ["loading"])]
pub fn line_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        LineSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// Two blobs that merge and separate as they cross, using an SVG gooey filter.
#[story(title = "Loading/LiquidDots", tags = ["loading"])]
pub fn liquid_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        LiquidDots {
            size: num(size, 24.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A shell prompt with a blinking block cursor.
#[story(title = "Loading/MacTerminal", tags = ["loading"])]
pub fn mac_terminal(prompt: String, color: String, duration: f64) -> Element {
    rsx! {
        MacTerminal {
            prompt: txt(prompt, "~ %"),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Two dots drawn together until they fuse, then pulled apart again.
#[story(title = "Loading/MagneticDots", tags = ["loading"])]
pub fn magnetic_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MagneticDots {
            size: num(size, 16.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A triangle outline drawn on, then drawn off the far side.
#[story(title = "Loading/MinimalTriangle", tags = ["loading"])]
pub fn minimal_triangle(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MinimalTriangle {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Four dots that draw together into a single blob and rotate apart again.
#[story(title = "Loading/MorphDotRing", tags = ["loading"])]
pub fn morph_dot_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MorphDotRing {
            size: num(size, 24.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A rounded square that morphs into a circle and back as it turns and pumps.
#[story(title = "Loading/MorphLoader", tags = ["loading"])]
pub fn morph_loader(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MorphLoader {
            size: num(size, 40.0),
            color: txt(color, "#3b82f6"),
            duration: num(duration, 2.2),
        }
    }
}

/// Three bars that shrink and slide together into a single block.
#[story(title = "Loading/MorphingBars", tags = ["loading"])]
pub fn morphing_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MorphingBars {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two rings trading places, each shrinking as the other grows.
#[story(title = "Loading/MorphingInfinity", tags = ["loading"])]
pub fn morphing_infinity(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MorphingInfinity {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// An outlined square rounding itself into a ring and back as it turns.
#[story(title = "Loading/MorphingRing", tags = ["loading"])]
pub fn morphing_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MorphingRing {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A solid square rounding into a disc and back while it turns and pulses.
#[story(title = "Loading/MorphingShape", tags = ["loading"])]
pub fn morphing_shape(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        MorphingShape {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Four suspended balls; the outermost two swing and hand off momentum.
#[story(title = "Loading/NewtonsCradle", tags = ["loading"])]
pub fn newtons_cradle(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        NewtonsCradle {
            size: num(size, 12.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Two arc pairs at right angles, easing around in opposite directions.
#[story(title = "Loading/OffsetRings", tags = ["loading"])]
pub fn offset_rings(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        OffsetRings {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A fixed core with two satellites orbiting it in opposite directions.
#[story(title = "Loading/OrbitingCircles", tags = ["loading"])]
pub fn orbiting_circles(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        OrbitingCircles {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A single bead running around a hairline track.
#[story(title = "Loading/OrbitingDot", tags = ["loading"])]
pub fn orbiting_dot(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        OrbitingDot {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two paper-like panels folding over their inner edges in unison.
#[story(title = "Loading/OrigamiShape", tags = ["loading"])]
pub fn origami_shape(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        OrigamiShape {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A weight on a rod swinging back and forth beneath a beam.
#[story(title = "Loading/Pendulum", tags = ["loading"])]
pub fn pendulum(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        Pendulum {
            size: num(size, 64.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Three dots swelling past their resting size in a rolling wave.
#[story(title = "Loading/PulsatingDots", tags = ["loading"])]
pub fn pulsating_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        PulsatingDots {
            size: num(size, 12.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// A status dot with two rings rippling outward from it.
#[story(title = "Loading/Pulse", tags = ["loading"])]
pub fn pulse(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        Pulse {
            size: num(size, 12.0),
            color: txt(color, "#3b82f6"),
            duration: num(duration, 1.8),
        }
    }
}

/// A dot with a single halo growing out of it and dissolving.
#[story(title = "Loading/PulseDot", tags = ["loading"])]
pub fn pulse_dot(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        PulseDot {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Three dots brightening and dimming in a rolling sequence.
#[story(title = "Loading/PulseDots", tags = ["loading"])]
pub fn pulse_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        PulseDots {
            size: num(size, 10.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.4),
        }
    }
}

/// An outlined square swelling into a faded ring and snapping back.
#[story(title = "Loading/PulseSquare", tags = ["loading"])]
pub fn pulse_square(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        PulseSquare {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A heart beating with the characteristic double thump.
#[story(title = "Loading/PumpingHeart", tags = ["loading"])]
pub fn pumping_heart(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        PumpingHeart {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A radar dish: concentric rings with a bright sweep hand circling the dial.
#[story(title = "Loading/RadarSweep", tags = ["loading"])]
pub fn radar_sweep(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        RadarSweep {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.8),
        }
    }
}

/// The plain spinner: a ring with one lit segment going round.
#[story(title = "Loading/RingSweep", tags = ["loading"])]
pub fn ring_sweep(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        RingSweep {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A dot dropping rings outward like a stone in water.
#[story(title = "Loading/RippleEffect", tags = ["loading"])]
pub fn ripple_effect(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        RippleEffect {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.2),
        }
    }
}

/// A plus sign flipping through a half turn, over and over.
#[story(title = "Loading/RotatingCross", tags = ["loading"])]
pub fn rotating_cross(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        RotatingCross {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.8),
        }
    }
}

/// A triangle outline turning at a constant rate.
#[story(title = "Loading/RotatingTriangle", tags = ["loading"])]
pub fn rotating_triangle(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        RotatingTriangle {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A two-by-two block whose tiles round off into dots in sequence.
#[story(title = "Loading/ShapeShiftGrid", tags = ["loading"])]
pub fn shape_shift_grid(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ShapeShiftGrid {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A slim indeterminate progress bar with a segment running along it.
#[story(title = "Loading/ShimmerLine", tags = ["loading"])]
pub fn shimmer_line(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ShimmerLine {
            size: num(size, 96.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Five bars rippling upward in sequence, like a voice assistant listening.
#[story(title = "Loading/SiriWave", tags = ["loading"])]
pub fn siri_wave(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SiriWave {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// A bare shimmering placeholder you size yourself.
#[story(title = "Loading/Skeleton", tags = ["loading"])]
pub fn skeleton(width: String, height: String, color: String, duration: f64) -> Element {
    rsx! {
        Skeleton {
            width: txt(width, "100%"),
            height: txt(height, "1rem"),
            color: txt(color, "rgba(255,255,255,.06)"),
            duration: num(duration, 1.6),
        }
    }
}

/// An avatar-and-lines placeholder, each row breathing a beat behind the last.
#[story(title = "Loading/SkeletonLoader", tags = ["loading"])]
pub fn skeleton_loader(width: String, duration: f64) -> Element {
    rsx! {
        SkeletonLoader {
            width: txt(width, "120px"),
            duration: num(duration, 1.5),
        }
    }
}

/// Two bars sliding past each other from opposite edges.
#[story(title = "Loading/SlidingBars", tags = ["loading"])]
pub fn sliding_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SlidingBars {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A filled dot hopping between three empty slots and back.
#[story(title = "Loading/SmoothDotShift", tags = ["loading"])]
pub fn smooth_dot_shift(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SmoothDotShift {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// A rounded arc on a faint track, turning smoothly.
#[story(title = "Loading/SmoothRing", tags = ["loading"])]
pub fn smooth_ring(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SmoothRing {
            size: num(size, 36.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// An outlined square softening into a circle and firming back up.
#[story(title = "Loading/SmoothRoundedSquare", tags = ["loading"])]
pub fn smooth_rounded_square(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SmoothRoundedSquare {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two squares tracing opposite corners of the same box.
#[story(title = "Loading/SpinningSquares", tags = ["loading"])]
pub fn spinning_squares(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SpinningSquares {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// An open ring that spins while drawing itself inward and out again.
#[story(title = "Loading/SpiralSpinner", tags = ["loading"])]
pub fn spiral_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SpiralSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Three stacked bars stretching out from the left in sequence.
#[story(title = "Loading/SpringBars", tags = ["loading"])]
pub fn spring_bars(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SpringBars {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.4),
        }
    }
}

/// A three-by-three dot matrix collapsing on a diagonal wave.
#[story(title = "Loading/SpringDotMatrix", tags = ["loading"])]
pub fn spring_dot_matrix(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SpringDotMatrix {
            size: num(size, 8.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A solid hexagon that swells and snaps a sixth of a turn.
#[story(title = "Loading/SpringHexagon", tags = ["loading"])]
pub fn spring_hexagon(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SpringHexagon {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two rings blooming outward from a point, half a beat apart.
#[story(title = "Loading/SpringRingExpand", tags = ["loading"])]
pub fn spring_ring_expand(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SpringRingExpand {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.6),
        }
    }
}

/// Text whose letters hop up one after another.
#[story(title = "Loading/SpringTextPop", tags = ["loading"])]
pub fn spring_text_pop(text: String, color: String, duration: f64, stagger: f64) -> Element {
    rsx! {
        SpringTextPop {
            text: txt(text, "Loading..."),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
            stagger: num(stagger, 0.08),
        }
    }
}

/// Three blocks stretching vertically in a rolling sequence.
#[story(title = "Loading/SquareAccordion", tags = ["loading"])]
pub fn square_accordion(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SquareAccordion {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Four squares blinking clockwise around a two-by-two grid.
#[story(title = "Loading/SquareGrid", tags = ["loading"])]
pub fn square_grid(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SquareGrid {
            size: num(size, 16.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A nine-tile block lighting up along a diagonal.
#[story(title = "Loading/SquareSnake", tags = ["loading"])]
pub fn square_snake(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SquareSnake {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// An outlined square stepping a quarter-turn at a time around a dot.
#[story(title = "Loading/SquareSpinner", tags = ["loading"])]
pub fn square_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SquareSpinner {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.6),
        }
    }
}

/// Three centred bars widening and brightening in sequence.
#[story(title = "Loading/StackedBarPulse", tags = ["loading"])]
pub fn stacked_bar_pulse(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        StackedBarPulse {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Two dots trading places and returning.
#[story(title = "Loading/SwappingDots", tags = ["loading"])]
pub fn swapping_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SwappingDots {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Two tapered rings spinning against each other, each fading around its arc.
#[story(title = "Loading/SwirlingSpinner", tags = ["loading"])]
pub fn swirling_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SwirlingSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.3),
        }
    }
}

/// Nine bars whose delays mirror around the centre, giving a symmetric ripple.
#[story(title = "Loading/SymmetricWave", tags = ["loading"])]
pub fn symmetric_wave(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        SymmetricWave {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}

/// A miniature terminal window with a blinking prompt cursor.
#[story(title = "Loading/TerminalLoader", tags = ["loading"])]
pub fn terminal_loader(
    command: String,
    background: String,
    color: String,
    accent_color: String,
    duration: f64,
) -> Element {
    rsx! {
        TerminalLoader {
            command: txt(command, "loading..."),
            background: txt(background, "#18181b"),
            color: txt(color, "#f4f4f5"),
            accent_color: txt(accent_color, "#34d399"),
            duration: num(duration, 0.8),
        }
    }
}

/// A word fading down and back up, over and over.
#[story(title = "Loading/TextBlink", tags = ["loading"])]
pub fn text_blink(text: String, color: String, duration: f64) -> Element {
    rsx! {
        TextBlink {
            text: txt(text, "Thinking"),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A label followed by an ellipsis that types itself in and clears.
#[story(title = "Loading/TextDots", tags = ["loading"])]
pub fn text_dots(text: String, color: String, duration: f64) -> Element {
    rsx! {
        TextDots {
            text: txt(text, "Thinking"),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two words alternating, each sliding out as the other slides in.
#[story(title = "Loading/TextMorph", tags = ["loading"])]
pub fn text_morph(
    first: String,
    second: String,
    width: String,
    color: String,
    duration: f64,
) -> Element {
    rsx! {
        TextMorph {
            first: txt(first, "Loading"),
            second: txt(second, "Wait"),
            width: txt(width, "96px"),
            color: txt(color, "currentColor"),
            duration: num(duration, 3.0),
        }
    }
}

/// Dimmed text with a bright band travelling across it.
#[story(title = "Loading/TextShimmer", tags = ["loading"])]
pub fn text_shimmer(text: String, color: String, duration: f64) -> Element {
    rsx! {
        TextShimmer {
            text: txt(text, "Thinking"),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Text whose letters brighten and lift in a travelling wave.
#[story(title = "Loading/TextShimmerWave", tags = ["loading"])]
pub fn text_shimmer_wave(text: String, color: String, duration: f64, stagger: f64) -> Element {
    rsx! {
        TextShimmerWave {
            text: txt(text, "Thinking"),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
            stagger: num(stagger, 0.1),
        }
    }
}

/// A lead dot orbiting with a fading tail of followers behind it.
#[story(title = "Loading/TrailingDots", tags = ["loading"])]
pub fn trailing_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        TrailingDots {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// Three dots evenly spaced on a ring, turning as one.
#[story(title = "Loading/TripleDotSpinner", tags = ["loading"])]
pub fn triple_dot_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        TripleDotSpinner {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.0),
        }
    }
}

/// Two half-rings nested inside each other, turning opposite ways.
#[story(title = "Loading/TwinOrbit", tags = ["loading"])]
pub fn twin_orbit(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        TwinOrbit {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.5),
        }
    }
}

/// A word followed by a blinking text caret.
#[story(title = "Loading/Typing", tags = ["loading"])]
pub fn typing(text: String, color: String, duration: f64) -> Element {
    rsx! {
        Typing {
            text: txt(text, "Loading"),
            color: txt(color, "currentColor"),
            duration: num(duration, 0.8),
        }
    }
}

/// The chat bubble with three dots that says someone is typing.
#[story(title = "Loading/TypingIndicator", tags = ["loading"])]
pub fn typing_indicator(size: f64, color: String, background: String, duration: f64) -> Element {
    rsx! {
        TypingIndicator {
            size: num(size, 6.0),
            color: txt(color, "currentColor"),
            background: txt(background, "rgba(128,128,128,.15)"),
            duration: num(duration, 0.6),
        }
    }
}

/// A square walking the perimeter of a box, tumbling as it goes.
#[story(title = "Loading/WanderingCube", tags = ["loading"])]
pub fn wandering_cube(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        WanderingCube {
            size: num(size, 40.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 2.5),
        }
    }
}

/// Three nested arcs sweeping at decreasing speeds, like watch complications.
#[story(title = "Loading/WatchSpinner", tags = ["loading"])]
pub fn watch_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        WatchSpinner {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Five dots riding a sine wave from left to right.
#[story(title = "Loading/WaveDots", tags = ["loading"])]
pub fn wave_dots(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        WaveDots {
            size: num(size, 8.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// A ball bouncing along a row of bars, deforming them as it passes.
#[story(title = "Loading/WavePhysicsLoader", tags = ["loading"])]
pub fn wave_physics_loader(scale: f64, color: String, duration: f64) -> Element {
    rsx! {
        WavePhysicsLoader {
            scale: num(scale, 1.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 4.0),
        }
    }
}

/// Eight bars whose offsets follow a sine curve, giving an irregular waveform.
#[story(title = "Loading/WaveformLoader", tags = ["loading"])]
pub fn waveform_loader(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        WaveformLoader {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}

/// Six dots placed on a zig-zag, lighting up along the path.
#[story(title = "Loading/ZigZagPulse", tags = ["loading"])]
pub fn zig_zag_pulse(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        ZigZagPulse {
            size: num(size, 48.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.2),
        }
    }
}
