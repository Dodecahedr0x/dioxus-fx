//! Stories for every loader in `dioxus_fx::loading`.
//!
//! Each story exposes the loader's own props as showcase controls; leaving a
//! control untouched renders the component's documented default.

use dioxus::prelude::*;
use dioxus_fx::loading::*;
use dioxus_showcase::prelude::*;

/// Four stacked bars that collapse and expand from the left in sequence.
#[story(title = "Loading/AccordionLoader", tags = ["loading"])]
pub fn accordion_loader(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        AccordionLoader {
            size,
            color,
            duration,
        }
    }
}

/// A rounded app tile with a progress ring sweeping inside it.
#[story(title = "Loading/AppIconLoad", tags = ["loading"])]
pub fn app_icon_load(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        AppIconLoad {
            size,
            color,
            duration,
        }
    }
}

/// Six translucent dots that drift outward and swell, then draw back in.
#[story(title = "Loading/AppleBreathe", tags = ["loading"])]
pub fn apple_breathe(
    #[default = 48.0] size: f64,
    #[default = "#2dd4bf"] color: String,
    #[default = "#22d3ee"] accent_color: String,
    #[default = 3.6] duration: f64,
) -> Element {
    rsx! {
        AppleBreathe {
            size,
            color,
            accent_color,
            duration,
        }
    }
}

/// Four audio-meter bars jumping at offset intervals.
#[story(title = "Loading/AppleEqualizer", tags = ["loading"])]
pub fn apple_equalizer(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.8] duration: f64,
) -> Element {
    rsx! {
        AppleEqualizer {
            size,
            color,
            duration,
        }
    }
}

/// A squircle that rounds itself into a circle and back while rotating.
#[story(title = "Loading/AppleIconMorph", tags = ["loading"])]
pub fn apple_icon_morph(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        AppleIconMorph {
            size,
            color,
            duration,
        }
    }
}

/// Three dots that swell and fade in a rolling sequence.
#[story(title = "Loading/ApplePulseDots", tags = ["loading"])]
pub fn apple_pulse_dots(
    #[default = 10.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        ApplePulseDots {
            size,
            color,
            duration,
        }
    }
}

/// A solid centre with two rings expanding out of it and fading.
#[story(title = "Loading/AppleScalePulse", tags = ["loading"])]
pub fn apple_scale_pulse(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        AppleScalePulse {
            size,
            color,
            duration,
        }
    }
}

/// Five bars forming a symmetric wave that swells from the centre outward.
#[story(title = "Loading/AppleSoundWave", tags = ["loading"])]
pub fn apple_sound_wave(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        AppleSoundWave {
            size,
            color,
            duration,
        }
    }
}

/// A word that scrolls up through a fixed-height window, over and over.
#[story(title = "Loading/AppleTextReveal", tags = ["loading"])]
pub fn apple_text_reveal(
    #[default = "Loading"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        AppleTextReveal {
            text,
            color,
            duration,
        }
    }
}

/// Dimmed text with a bright highlight sweeping across it, iOS lock-screen style.
#[story(title = "Loading/AppleUnlock", tags = ["loading"])]
pub fn apple_unlock(
    #[default = "Slide to unlock"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 2.2] duration: f64,
) -> Element {
    rsx! {
        AppleUnlock {
            text,
            color,
            duration,
        }
    }
}

/// An arc that draws itself around a track, then unwinds off the other end.
#[story(title = "Loading/ArcTracer", tags = ["loading"])]
pub fn arc_tracer(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        ArcTracer {
            size,
            color,
            duration,
        }
    }
}

/// Five bars rising and falling in a cascading run.
#[story(title = "Loading/BarCascade", tags = ["loading"])]
pub fn bar_cascade(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        BarCascade {
            size,
            color,
            duration,
        }
    }
}

/// A bead sliding from end to end inside a pill-shaped track.
#[story(title = "Loading/BarSweep", tags = ["loading"])]
pub fn bar_sweep(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        BarSweep {
            size,
            color,
            duration,
        }
    }
}

/// Three dots bobbing up and down in a rolling wave.
#[story(title = "Loading/BobbingDots", tags = ["loading"])]
pub fn bobbing_dots(
    #[default = 12.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        BobbingDots {
            size,
            color,
            duration,
        }
    }
}

/// Three small dots hopping in quick succession.
#[story(title = "Loading/BounceDots", tags = ["loading"])]
pub fn bounce_dots(
    #[default = 10.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.6] duration: f64,
) -> Element {
    rsx! {
        BounceDots {
            size,
            color,
            duration,
        }
    }
}

/// Three bars squeezing and releasing along their length.
#[story(title = "Loading/BouncingBars", tags = ["loading"])]
pub fn bouncing_bars(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        BouncingBars {
            size,
            color,
            duration,
        }
    }
}

/// Three dots bouncing off a baseline, stretching as they leave it.
#[story(title = "Loading/BouncingDots", tags = ["loading"])]
pub fn bouncing_dots(
    #[default = 64.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.8] duration: f64,
) -> Element {
    rsx! {
        BouncingDots {
            size,
            color,
            duration,
        }
    }
}

/// Three stacked lines extending from the left in sequence.
#[story(title = "Loading/BouncingLines", tags = ["loading"])]
pub fn bouncing_lines(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        BouncingLines {
            size,
            color,
            duration,
        }
    }
}

/// A square bouncing on its shadow, squashing on impact.
#[story(title = "Loading/BouncingSquare", tags = ["loading"])]
pub fn bouncing_square(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.6] duration: f64,
) -> Element {
    rsx! {
        BouncingSquare {
            size,
            color,
            duration,
        }
    }
}

/// A ring that swells as its stroke thins, then draws back in.
#[story(title = "Loading/BreatheRing", tags = ["loading"])]
pub fn breathe_ring(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 3.0] duration: f64,
) -> Element {
    rsx! {
        BreatheRing {
            size,
            color,
            duration,
        }
    }
}

/// A bright core sitting inside a halo that breathes in and out.
#[story(title = "Loading/BreathingGlow", tags = ["loading"])]
pub fn breathing_glow(
    #[default = 48.0] size: f64,
    #[default = "#ffffff"] color: String,
    #[default = "#3b82f6"] glow_color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        BreathingGlow {
            size,
            color,
            glow_color,
            duration,
        }
    }
}

/// A solid square inflating into a disc and back as it turns.
#[story(title = "Loading/BreathingSquare", tags = ["loading"])]
pub fn breathing_square(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        BreathingSquare {
            size,
            color,
            duration,
        }
    }
}

/// Eight radial spokes that stretch and shrink around the dial in sequence.
#[story(title = "Loading/CircularBars", tags = ["loading"])]
pub fn circular_bars(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        CircularBars {
            size,
            color,
            duration,
        }
    }
}

/// The twelve-spoke system spinner, each spoke fading in turn.
#[story(title = "Loading/ClassicSpinner", tags = ["loading"])]
pub fn classic_spinner(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        ClassicSpinner {
            size,
            color,
            duration,
        }
    }
}

/// A clock face whose two hands sweep at a 6:1 ratio.
#[story(title = "Loading/ClockSpinner", tags = ["loading"])]
pub fn clock_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        ClockSpinner {
            size,
            color,
            duration,
        }
    }
}

/// A ring with a comet tail that fades from solid to nothing as it spins.
#[story(title = "Loading/CometSpinner", tags = ["loading"])]
pub fn comet_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 4.0] thickness: f64,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        CometSpinner {
            size,
            color,
            thickness,
            duration,
        }
    }
}

/// Three hairline rings expanding out of a single point, like sonar.
#[story(title = "Loading/ConcentricPulse", tags = ["loading"])]
pub fn concentric_pulse(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        ConcentricPulse {
            size,
            color,
            duration,
        }
    }
}

/// Three nested rings turning at different speeds, the middle one reversed.
#[story(title = "Loading/ConcentricRing", tags = ["loading"])]
pub fn concentric_ring(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        ConcentricRing {
            size,
            color,
            duration,
        }
    }
}

/// Two nested squares turning against each other, one growing as the other shrinks.
#[story(title = "Loading/ConcentricSquares", tags = ["loading"])]
pub fn concentric_squares(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        ConcentricSquares {
            size,
            color,
            duration,
        }
    }
}

/// A track of beads sliding endlessly to the left inside a pill.
#[story(title = "Loading/ConveyorLoop", tags = ["loading"])]
pub fn conveyor_loop(
    #[default = 64.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        ConveyorLoop {
            size,
            color,
            duration,
        }
    }
}

/// A plus sign that steps around a full turn in quarters.
#[story(title = "Loading/CrossSpinner", tags = ["loading"])]
pub fn cross_spinner(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        CrossSpinner {
            size,
            color,
            duration,
        }
    }
}

/// A rounded tile flipping over one axis and then the other, with a little bounce.
#[story(title = "Loading/CubeFlipSpring", tags = ["loading"])]
pub fn cube_flip_spring(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        CubeFlipSpring {
            size,
            color,
            duration,
        }
    }
}

/// A dashed circle rotating slowly, like a selection marquee.
#[story(title = "Loading/DashRing", tags = ["loading"])]
pub fn dash_ring(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 4.0] duration: f64,
) -> Element {
    rsx! {
        DashRing {
            size,
            color,
            duration,
        }
    }
}

/// A dashed ring turning while it breathes in and out.
#[story(title = "Loading/DashedSpiral", tags = ["loading"])]
pub fn dashed_spiral(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 4.0] duration: f64,
) -> Element {
    rsx! {
        DashedSpiral {
            size,
            color,
            duration,
        }
    }
}

/// Four tiles arranged as a diamond, blinking in a rolling sequence.
#[story(title = "Loading/DiamondGrid", tags = ["loading"])]
pub fn diamond_grid(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        DiamondGrid {
            size,
            color,
            duration,
        }
    }
}

/// A diamond that snaps a quarter-turn at a time, overshooting each stop.
#[story(title = "Loading/DiamondRotateSpring", tags = ["loading"])]
pub fn diamond_rotate_spring(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        DiamondRotateSpring {
            size,
            color,
            duration,
        }
    }
}

/// Eight dots on a ring, each dimming a beat after the last.
#[story(title = "Loading/DotSpinner", tags = ["loading"])]
pub fn dot_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        DotSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Eight dots on a ring that shrink and fade in a travelling wave.
#[story(title = "Loading/DotsRing", tags = ["loading"])]
pub fn dots_ring(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        DotsRing {
            size,
            color,
            duration,
        }
    }
}

/// Two rings chasing each other in opposite directions.
#[story(title = "Loading/DoubleRing", tags = ["loading"])]
pub fn double_ring(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        DoubleRing {
            size,
            color,
            duration,
        }
    }
}

/// A droplet that falls onto a soft shadow, squashing as it lands.
#[story(title = "Loading/DropDot", tags = ["loading"])]
pub fn drop_dot(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        DropDot {
            size,
            color,
            duration,
        }
    }
}

/// Two opposing arcs spinning while the ring pumps in and out.
#[story(title = "Loading/DualArc", tags = ["loading"])]
pub fn dual_arc(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        DualArc {
            size,
            color,
            duration,
        }
    }
}

/// The iOS pill that widens and narrows with a live-activity indicator inside.
#[story(title = "Loading/DynamicIsland", tags = ["loading"])]
pub fn dynamic_island(
    #[default = "#09090b"] color: String,
    #[default = "#10b981"] accent_color: String,
    #[default = 2.2] duration: f64,
) -> Element {
    rsx! {
        DynamicIsland {
            color,
            accent_color,
            duration,
        }
    }
}

/// Three bars stretching tall and snapping back, one after another.
#[story(title = "Loading/ElasticBars", tags = ["loading"])]
pub fn elastic_bars(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.1] duration: f64,
) -> Element {
    rsx! {
        ElasticBars {
            size,
            color,
            duration,
        }
    }
}

/// A squishy square bouncing off a baseline.
#[story(title = "Loading/ElasticSquare", tags = ["loading"])]
pub fn elastic_square(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.8] duration: f64,
) -> Element {
    rsx! {
        ElasticSquare {
            size,
            color,
            duration,
        }
    }
}

/// A cross whose arms take turns stretching to full width and height.
#[story(title = "Loading/ExpandingCross", tags = ["loading"])]
pub fn expanding_cross(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        ExpandingCross {
            size,
            color,
            duration,
        }
    }
}

/// A framed face outline with a scan line sweeping down and back.
#[story(title = "Loading/FaceIdScan", tags = ["loading"])]
pub fn face_id_scan(
    #[default = 48.0] size: f64,
    #[default = "#22c55e"] color: String,
    #[default = "rgba(74,222,128,.3)"] scan_color: String,
    #[default = 2.5] duration: f64,
) -> Element {
    rsx! {
        FaceIdScan {
            size,
            color,
            scan_color,
            duration,
        }
    }
}

/// A fixed arc riding a faint track, spinning at a constant rate.
#[story(title = "Loading/FadeArc", tags = ["loading"])]
pub fn fade_arc(
    #[default = 36.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        FadeArc {
            size,
            color,
            duration,
        }
    }
}

/// Four dots fading up and out in a marching sequence.
#[story(title = "Loading/FadeDots", tags = ["loading"])]
pub fn fade_dots(
    #[default = 10.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        FadeDots {
            size,
            color,
            duration,
        }
    }
}

/// A tile flipping through all four faces of a two-axis cycle.
#[story(title = "Loading/FlipSquare", tags = ["loading"])]
pub fn flip_square(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        FlipSquare {
            size,
            color,
            duration,
        }
    }
}

/// Three diamonds bobbing up and down in a rolling wave.
#[story(title = "Loading/FloatingDiamonds", tags = ["loading"])]
pub fn floating_diamonds(
    #[default = 12.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        FloatingDiamonds {
            size,
            color,
            duration,
        }
    }
}

/// Four bars swaying like reeds, each a beat behind the last.
#[story(title = "Loading/FluidBars", tags = ["loading"])]
pub fn fluid_bars(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        FluidBars {
            size,
            color,
            duration,
        }
    }
}

/// A diamond flattening and rebounding, as if made of liquid.
#[story(title = "Loading/FluidDiamond", tags = ["loading"])]
pub fn fluid_diamond(
    #[default = 24.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        FluidDiamond {
            size,
            color,
            duration,
        }
    }
}

/// A still centre dot with a smaller satellite circling it.
#[story(title = "Loading/FluidDotOrbit", tags = ["loading"])]
pub fn fluid_dot_orbit(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        FluidDotOrbit {
            size,
            color,
            duration,
        }
    }
}

/// A rounded placeholder block with a highlight sweeping across it.
#[story(title = "Loading/FluidSkeleton", tags = ["loading"])]
pub fn fluid_skeleton(
    #[default = "96px"] width: String,
    #[default = "40px"] height: String,
    #[default = "12px"] radius: String,
    #[default = "rgba(255,255,255,.6)"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        FluidSkeleton {
            width,
            height,
            radius,
            color,
            duration,
        }
    }
}

/// Two interlocking dashed cogs turning against each other.
#[story(title = "Loading/Gears", tags = ["loading"])]
pub fn gears(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 4.0] duration: f64,
) -> Element {
    rsx! {
        Gears {
            size,
            color,
            duration,
        }
    }
}

/// A frosted glass panel with a drifting glow behind a small spinner.
#[story(title = "Loading/GlassmorphicCard", tags = ["loading"])]
pub fn glassmorphic_card(
    #[default = "80px"] width: String,
    #[default = "64px"] height: String,
    #[default = "currentColor"] color: String,
    #[default = "rgba(161,161,170,.65)"] glow_color: String,
    #[default = 3.0] duration: f64,
) -> Element {
    rsx! {
        GlassmorphicCard {
            width,
            height,
            color,
            glow_color,
            duration,
        }
    }
}

/// A gradient-filled ring spinning at a constant rate.
#[story(title = "Loading/GradientArc", tags = ["loading"])]
pub fn gradient_arc(
    #[default = 40.0] size: f64,
    #[default = "#00f2fe"] from_color: String,
    #[default = "#4facfe"] to_color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        GradientArc {
            size,
            from_color,
            to_color,
            duration,
        }
    }
}

/// A three-by-three field of dots pulsing on a diagonal wave.
#[story(title = "Loading/GridDots", tags = ["loading"])]
pub fn grid_dots(
    #[default = 10.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        GridDots {
            size,
            color,
            duration,
        }
    }
}

/// A ring that snaps through quarter turns with a springy overshoot.
#[story(title = "Loading/HapticRing", tags = ["loading"])]
pub fn haptic_ring(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        HapticRing {
            size,
            color,
            duration,
        }
    }
}

/// An ECG trace scrolling across the screen.
#[story(title = "Loading/Heartbeat", tags = ["loading"])]
pub fn heartbeat(
    #[default = 64.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        Heartbeat {
            size,
            color,
            duration,
        }
    }
}

/// A hexagon outline being traced by a travelling stroke.
#[story(title = "Loading/HexagonSpinner", tags = ["loading"])]
pub fn hexagon_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        HexagonSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Two stacked triangles flipping end over end, like sand running out.
#[story(title = "Loading/Hourglass", tags = ["loading"])]
pub fn hourglass(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 3.0] duration: f64,
) -> Element {
    rsx! {
        Hourglass {
            size,
            color,
            duration,
        }
    }
}

/// A stroke chasing itself around a lemniscate.
#[story(title = "Loading/InfinityPath", tags = ["loading"])]
pub fn infinity_path(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        InfinityPath {
            size,
            color,
            duration,
        }
    }
}

/// Two rings tumbling through each other in three dimensions.
#[story(title = "Loading/IntersectingRings", tags = ["loading"])]
pub fn intersecting_rings(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.2] duration: f64,
) -> Element {
    rsx! {
        IntersectingRings {
            size,
            color,
            duration,
        }
    }
}

/// The iOS activity indicator: twelve tapered spokes fading around the dial.
#[story(title = "Loading/IosSpinner", tags = ["loading"])]
pub fn ios_spinner(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        IosSpinner {
            size,
            color,
            duration,
        }
    }
}

/// A single needle sweeping inside a hairline circle.
#[story(title = "Loading/LineSpinner", tags = ["loading"])]
pub fn line_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        LineSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Two blobs that merge and separate as they cross, using an SVG gooey filter.
#[story(title = "Loading/LiquidDots", tags = ["loading"])]
pub fn liquid_dots(
    #[default = 24.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        LiquidDots {
            size,
            color,
            duration,
        }
    }
}

/// A shell prompt with a blinking block cursor.
#[story(title = "Loading/MacTerminal", tags = ["loading"])]
pub fn mac_terminal(
    #[default = "~ %"] prompt: String,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        MacTerminal {
            prompt,
            color,
            duration,
        }
    }
}

/// Two dots drawn together until they fuse, then pulled apart again.
#[story(title = "Loading/MagneticDots", tags = ["loading"])]
pub fn magnetic_dots(
    #[default = 16.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        MagneticDots {
            size,
            color,
            duration,
        }
    }
}

/// A triangle outline drawn on, then drawn off the far side.
#[story(title = "Loading/MinimalTriangle", tags = ["loading"])]
pub fn minimal_triangle(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        MinimalTriangle {
            size,
            color,
            duration,
        }
    }
}

/// Four dots that draw together into a single blob and rotate apart again.
#[story(title = "Loading/MorphDotRing", tags = ["loading"])]
pub fn morph_dot_ring(
    #[default = 24.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        MorphDotRing {
            size,
            color,
            duration,
        }
    }
}

/// A rounded square that morphs into a circle and back as it turns and pumps.
#[story(title = "Loading/MorphLoader", tags = ["loading"])]
pub fn morph_loader(
    #[default = 40.0] size: f64,
    #[default = "#3b82f6"] color: String,
    #[default = 2.2] duration: f64,
) -> Element {
    rsx! {
        MorphLoader {
            size,
            color,
            duration,
        }
    }
}

/// Three bars that shrink and slide together into a single block.
#[story(title = "Loading/MorphingBars", tags = ["loading"])]
pub fn morphing_bars(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        MorphingBars {
            size,
            color,
            duration,
        }
    }
}

/// Two rings trading places, each shrinking as the other grows.
#[story(title = "Loading/MorphingInfinity", tags = ["loading"])]
pub fn morphing_infinity(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        MorphingInfinity {
            size,
            color,
            duration,
        }
    }
}

/// An outlined square rounding itself into a ring and back as it turns.
#[story(title = "Loading/MorphingRing", tags = ["loading"])]
pub fn morphing_ring(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        MorphingRing {
            size,
            color,
            duration,
        }
    }
}

/// A solid square rounding into a disc and back while it turns and pulses.
#[story(title = "Loading/MorphingShape", tags = ["loading"])]
pub fn morphing_shape(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        MorphingShape {
            size,
            color,
            duration,
        }
    }
}

/// Four suspended balls; the outermost two swing and hand off momentum.
#[story(title = "Loading/NewtonsCradle", tags = ["loading"])]
pub fn newtons_cradle(
    #[default = 12.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        NewtonsCradle {
            size,
            color,
            duration,
        }
    }
}

/// Two arc pairs at right angles, easing around in opposite directions.
#[story(title = "Loading/OffsetRings", tags = ["loading"])]
pub fn offset_rings(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        OffsetRings {
            size,
            color,
            duration,
        }
    }
}

/// A fixed core with two satellites orbiting it in opposite directions.
#[story(title = "Loading/OrbitingCircles", tags = ["loading"])]
pub fn orbiting_circles(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        OrbitingCircles {
            size,
            color,
            duration,
        }
    }
}

/// A single bead running around a hairline track.
#[story(title = "Loading/OrbitingDot", tags = ["loading"])]
pub fn orbiting_dot(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        OrbitingDot {
            size,
            color,
            duration,
        }
    }
}

/// Two paper-like panels folding over their inner edges in unison.
#[story(title = "Loading/OrigamiShape", tags = ["loading"])]
pub fn origami_shape(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        OrigamiShape {
            size,
            color,
            duration,
        }
    }
}

/// A weight on a rod swinging back and forth beneath a beam.
#[story(title = "Loading/Pendulum", tags = ["loading"])]
pub fn pendulum(
    #[default = 64.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        Pendulum {
            size,
            color,
            duration,
        }
    }
}

/// Three dots swelling past their resting size in a rolling wave.
#[story(title = "Loading/PulsatingDots", tags = ["loading"])]
pub fn pulsating_dots(
    #[default = 12.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        PulsatingDots {
            size,
            color,
            duration,
        }
    }
}

/// A status dot with two rings rippling outward from it.
#[story(title = "Loading/Pulse", tags = ["loading"])]
pub fn pulse(
    #[default = 12.0] size: f64,
    #[default = "#3b82f6"] color: String,
    #[default = 1.8] duration: f64,
) -> Element {
    rsx! {
        Pulse {
            size,
            color,
            duration,
        }
    }
}

/// A dot with a single halo growing out of it and dissolving.
#[story(title = "Loading/PulseDot", tags = ["loading"])]
pub fn pulse_dot(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        PulseDot {
            size,
            color,
            duration,
        }
    }
}

/// Three dots brightening and dimming in a rolling sequence.
#[story(title = "Loading/PulseDots", tags = ["loading"])]
pub fn pulse_dots(
    #[default = 10.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.4] duration: f64,
) -> Element {
    rsx! {
        PulseDots {
            size,
            color,
            duration,
        }
    }
}

/// An outlined square swelling into a faded ring and snapping back.
#[story(title = "Loading/PulseSquare", tags = ["loading"])]
pub fn pulse_square(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        PulseSquare {
            size,
            color,
            duration,
        }
    }
}

/// A heart beating with the characteristic double thump.
#[story(title = "Loading/PumpingHeart", tags = ["loading"])]
pub fn pumping_heart(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        PumpingHeart {
            size,
            color,
            duration,
        }
    }
}

/// A radar dish: concentric rings with a bright sweep hand circling the dial.
#[story(title = "Loading/RadarSweep", tags = ["loading"])]
pub fn radar_sweep(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.8] duration: f64,
) -> Element {
    rsx! {
        RadarSweep {
            size,
            color,
            duration,
        }
    }
}

/// The plain spinner: a ring with one lit segment going round.
#[story(title = "Loading/RingSweep", tags = ["loading"])]
pub fn ring_sweep(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        RingSweep {
            size,
            color,
            duration,
        }
    }
}

/// A dot dropping rings outward like a stone in water.
#[story(title = "Loading/RippleEffect", tags = ["loading"])]
pub fn ripple_effect(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.2] duration: f64,
) -> Element {
    rsx! {
        RippleEffect {
            size,
            color,
            duration,
        }
    }
}

/// A plus sign flipping through a half turn, over and over.
#[story(title = "Loading/RotatingCross", tags = ["loading"])]
pub fn rotating_cross(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.8] duration: f64,
) -> Element {
    rsx! {
        RotatingCross {
            size,
            color,
            duration,
        }
    }
}

/// A triangle outline turning at a constant rate.
#[story(title = "Loading/RotatingTriangle", tags = ["loading"])]
pub fn rotating_triangle(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        RotatingTriangle {
            size,
            color,
            duration,
        }
    }
}

/// A two-by-two block whose tiles round off into dots in sequence.
#[story(title = "Loading/ShapeShiftGrid", tags = ["loading"])]
pub fn shape_shift_grid(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        ShapeShiftGrid {
            size,
            color,
            duration,
        }
    }
}

/// A slim indeterminate progress bar with a segment running along it.
#[story(title = "Loading/ShimmerLine", tags = ["loading"])]
pub fn shimmer_line(
    #[default = 96.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        ShimmerLine {
            size,
            color,
            duration,
        }
    }
}

/// Five bars rippling upward in sequence, like a voice assistant listening.
#[story(title = "Loading/SiriWave", tags = ["loading"])]
pub fn siri_wave(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        SiriWave {
            size,
            color,
            duration,
        }
    }
}

/// A bare shimmering placeholder you size yourself.
#[story(title = "Loading/Skeleton", tags = ["loading"])]
pub fn skeleton(
    #[default = "100%"] width: String,
    #[default = "1rem"] height: String,
    #[default = "rgba(255,255,255,.06)"] color: String,
    #[default = 1.6] duration: f64,
) -> Element {
    rsx! {
        Skeleton {
            width,
            height,
            color,
            duration,
        }
    }
}

/// An avatar-and-lines placeholder, each row breathing a beat behind the last.
#[story(title = "Loading/SkeletonLoader", tags = ["loading"])]
pub fn skeleton_loader(
    #[default = "120px"] width: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SkeletonLoader {
            width,
            duration,
        }
    }
}

/// Two bars sliding past each other from opposite edges.
#[story(title = "Loading/SlidingBars", tags = ["loading"])]
pub fn sliding_bars(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SlidingBars {
            size,
            color,
            duration,
        }
    }
}

/// A filled dot hopping between three empty slots and back.
#[story(title = "Loading/SmoothDotShift", tags = ["loading"])]
pub fn smooth_dot_shift(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        SmoothDotShift {
            size,
            color,
            duration,
        }
    }
}

/// A rounded arc on a faint track, turning smoothly.
#[story(title = "Loading/SmoothRing", tags = ["loading"])]
pub fn smooth_ring(
    #[default = 36.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        SmoothRing {
            size,
            color,
            duration,
        }
    }
}

/// An outlined square softening into a circle and firming back up.
#[story(title = "Loading/SmoothRoundedSquare", tags = ["loading"])]
pub fn smooth_rounded_square(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        SmoothRoundedSquare {
            size,
            color,
            duration,
        }
    }
}

/// Two squares tracing opposite corners of the same box.
#[story(title = "Loading/SpinningSquares", tags = ["loading"])]
pub fn spinning_squares(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        SpinningSquares {
            size,
            color,
            duration,
        }
    }
}

/// An open ring that spins while drawing itself inward and out again.
#[story(title = "Loading/SpiralSpinner", tags = ["loading"])]
pub fn spiral_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SpiralSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Three stacked bars stretching out from the left in sequence.
#[story(title = "Loading/SpringBars", tags = ["loading"])]
pub fn spring_bars(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.4] duration: f64,
) -> Element {
    rsx! {
        SpringBars {
            size,
            color,
            duration,
        }
    }
}

/// A three-by-three dot matrix collapsing on a diagonal wave.
#[story(title = "Loading/SpringDotMatrix", tags = ["loading"])]
pub fn spring_dot_matrix(
    #[default = 8.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SpringDotMatrix {
            size,
            color,
            duration,
        }
    }
}

/// A solid hexagon that swells and snaps a sixth of a turn.
#[story(title = "Loading/SpringHexagon", tags = ["loading"])]
pub fn spring_hexagon(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        SpringHexagon {
            size,
            color,
            duration,
        }
    }
}

/// Two rings blooming outward from a point, half a beat apart.
#[story(title = "Loading/SpringRingExpand", tags = ["loading"])]
pub fn spring_ring_expand(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.6] duration: f64,
) -> Element {
    rsx! {
        SpringRingExpand {
            size,
            color,
            duration,
        }
    }
}

/// Text whose letters hop up one after another.
#[story(title = "Loading/SpringTextPop", tags = ["loading"])]
pub fn spring_text_pop(
    #[default = "Loading..."] text: String,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
    #[default = 0.08] stagger: f64,
) -> Element {
    rsx! {
        SpringTextPop {
            text,
            color,
            duration,
            stagger,
        }
    }
}

/// Three blocks stretching vertically in a rolling sequence.
#[story(title = "Loading/SquareAccordion", tags = ["loading"])]
pub fn square_accordion(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SquareAccordion {
            size,
            color,
            duration,
        }
    }
}

/// Four squares blinking clockwise around a two-by-two grid.
#[story(title = "Loading/SquareGrid", tags = ["loading"])]
pub fn square_grid(
    #[default = 16.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SquareGrid {
            size,
            color,
            duration,
        }
    }
}

/// A nine-tile block lighting up along a diagonal.
#[story(title = "Loading/SquareSnake", tags = ["loading"])]
pub fn square_snake(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SquareSnake {
            size,
            color,
            duration,
        }
    }
}

/// An outlined square stepping a quarter-turn at a time around a dot.
#[story(title = "Loading/SquareSpinner", tags = ["loading"])]
pub fn square_spinner(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 0.6] duration: f64,
) -> Element {
    rsx! {
        SquareSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Three centred bars widening and brightening in sequence.
#[story(title = "Loading/StackedBarPulse", tags = ["loading"])]
pub fn stacked_bar_pulse(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        StackedBarPulse {
            size,
            color,
            duration,
        }
    }
}

/// Two dots trading places and returning.
#[story(title = "Loading/SwappingDots", tags = ["loading"])]
pub fn swapping_dots(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        SwappingDots {
            size,
            color,
            duration,
        }
    }
}

/// Two tapered rings spinning against each other, each fading around its arc.
#[story(title = "Loading/SwirlingSpinner", tags = ["loading"])]
pub fn swirling_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.3] duration: f64,
) -> Element {
    rsx! {
        SwirlingSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Nine bars whose delays mirror around the centre, giving a symmetric ripple.
#[story(title = "Loading/SymmetricWave", tags = ["loading"])]
pub fn symmetric_wave(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        SymmetricWave {
            size,
            color,
            duration,
        }
    }
}

/// A miniature terminal window with a blinking prompt cursor.
#[story(title = "Loading/TerminalLoader", tags = ["loading"])]
pub fn terminal_loader(
    #[default = "loading..."] command: String,
    #[default = "#18181b"] background: String,
    #[default = "#f4f4f5"] color: String,
    #[default = "#34d399"] accent_color: String,
    #[default = 0.8] duration: f64,
) -> Element {
    rsx! {
        TerminalLoader {
            command,
            background,
            color,
            accent_color,
            duration,
        }
    }
}

/// A word fading down and back up, over and over.
#[story(title = "Loading/TextBlink", tags = ["loading"])]
pub fn text_blink(
    #[default = "Thinking"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        TextBlink {
            text,
            color,
            duration,
        }
    }
}

/// A label followed by an ellipsis that types itself in and clears.
#[story(title = "Loading/TextDots", tags = ["loading"])]
pub fn text_dots(
    #[default = "Thinking"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        TextDots {
            text,
            color,
            duration,
        }
    }
}

/// Two words alternating, each sliding out as the other slides in.
#[story(title = "Loading/TextMorph", tags = ["loading"])]
pub fn text_morph(
    #[default = "Loading"] first: String,
    #[default = "Wait"] second: String,
    #[default = "96px"] width: String,
    #[default = "currentColor"] color: String,
    #[default = 3.0] duration: f64,
) -> Element {
    rsx! {
        TextMorph {
            first,
            second,
            width,
            color,
            duration,
        }
    }
}

/// Dimmed text with a bright band travelling across it.
#[story(title = "Loading/TextShimmer", tags = ["loading"])]
pub fn text_shimmer(
    #[default = "Thinking"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        TextShimmer {
            text,
            color,
            duration,
        }
    }
}

/// Text whose letters brighten and lift in a travelling wave.
#[story(title = "Loading/TextShimmerWave", tags = ["loading"])]
pub fn text_shimmer_wave(
    #[default = "Thinking"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
    #[default = 0.1] stagger: f64,
) -> Element {
    rsx! {
        TextShimmerWave {
            text,
            color,
            duration,
            stagger,
        }
    }
}

/// A lead dot orbiting with a fading tail of followers behind it.
#[story(title = "Loading/TrailingDots", tags = ["loading"])]
pub fn trailing_dots(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        TrailingDots {
            size,
            color,
            duration,
        }
    }
}

/// Three dots evenly spaced on a ring, turning as one.
#[story(title = "Loading/TripleDotSpinner", tags = ["loading"])]
pub fn triple_dot_spinner(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.0] duration: f64,
) -> Element {
    rsx! {
        TripleDotSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Two half-rings nested inside each other, turning opposite ways.
#[story(title = "Loading/TwinOrbit", tags = ["loading"])]
pub fn twin_orbit(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.5] duration: f64,
) -> Element {
    rsx! {
        TwinOrbit {
            size,
            color,
            duration,
        }
    }
}

/// A word followed by a blinking text caret.
#[story(title = "Loading/Typing", tags = ["loading"])]
pub fn typing(
    #[default = "Loading"] text: String,
    #[default = "currentColor"] color: String,
    #[default = 0.8] duration: f64,
) -> Element {
    rsx! {
        Typing {
            text,
            color,
            duration,
        }
    }
}

/// The chat bubble with three dots that says someone is typing.
#[story(title = "Loading/TypingIndicator", tags = ["loading"])]
pub fn typing_indicator(
    #[default = 6.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = "rgba(128,128,128,.15)"] background: String,
    #[default = 0.6] duration: f64,
) -> Element {
    rsx! {
        TypingIndicator {
            size,
            color,
            background,
            duration,
        }
    }
}

/// A square walking the perimeter of a box, tumbling as it goes.
#[story(title = "Loading/WanderingCube", tags = ["loading"])]
pub fn wandering_cube(
    #[default = 40.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 2.5] duration: f64,
) -> Element {
    rsx! {
        WanderingCube {
            size,
            color,
            duration,
        }
    }
}

/// Three nested arcs sweeping at decreasing speeds, like watch complications.
#[story(title = "Loading/WatchSpinner", tags = ["loading"])]
pub fn watch_spinner(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        WatchSpinner {
            size,
            color,
            duration,
        }
    }
}

/// Five dots riding a sine wave from left to right.
#[story(title = "Loading/WaveDots", tags = ["loading"])]
pub fn wave_dots(
    #[default = 8.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        WaveDots {
            size,
            color,
            duration,
        }
    }
}

/// A ball bouncing along a row of bars, deforming them as it passes.
#[story(title = "Loading/WavePhysicsLoader", tags = ["loading"])]
pub fn wave_physics_loader(
    #[default = 1.0] scale: f64,
    #[default = "currentColor"] color: String,
    #[default = 4.0] duration: f64,
) -> Element {
    rsx! {
        WavePhysicsLoader {
            scale,
            color,
            duration,
        }
    }
}

/// Eight bars whose offsets follow a sine curve, giving an irregular waveform.
#[story(title = "Loading/WaveformLoader", tags = ["loading"])]
pub fn waveform_loader(
    #[default = 32.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.0] duration: f64,
) -> Element {
    rsx! {
        WaveformLoader {
            size,
            color,
            duration,
        }
    }
}

/// Six dots placed on a zig-zag, lighting up along the path.
#[story(title = "Loading/ZigZagPulse", tags = ["loading"])]
pub fn zig_zag_pulse(
    #[default = 48.0] size: f64,
    #[default = "currentColor"] color: String,
    #[default = 1.2] duration: f64,
) -> Element {
    rsx! {
        ZigZagPulse {
            size,
            color,
            duration,
        }
    }
}
