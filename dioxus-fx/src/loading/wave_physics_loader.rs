use crate::style::BASE_CSS;
use dioxus::prelude::*;
use std::sync::OnceLock;

/// Layout rules. The keyframes are computed rather than written by hand — see
/// [`keyframes`].
pub(crate) const CSS: &str = r#"
.dfx-wave-physics{display:flex;flex-direction:column;align-items:center;justify-content:center;transform:scale(var(--dfx-scale));transform-origin:center}
.dfx-wave-physics div{position:relative;display:flex;align-items:flex-end;justify-content:flex-start;gap:8px;width:292px;height:192px}
.dfx-wave-physics i{width:12px;height:16px;border-radius:9999px;background:var(--dfx-track);transform-origin:bottom center;animation-duration:var(--dfx-duration);animation-timing-function:linear;animation-iteration-count:infinite}
.dfx-wave-physics b{position:absolute;bottom:0;left:0;z-index:1;width:12px;height:12px;border-radius:9999px;background:var(--dfx-color);transform-origin:bottom center;animation:dfx-wave-physics-ball var(--dfx-duration) linear infinite}
"#;

// Geometry of the original component, in pixels.
const BARS: usize = 15;
const PITCH: f64 = 20.0; // bar width (12) + gap (8)
const BASE_H: f64 = 16.0;
const PEAK_H: f64 = 48.0;
const MAX_BOUNCE: f64 = 60.0;
const BOUNCES: f64 = 4.0; // bounces per one-way traverse
const FRAMES: usize = 41;

/// State of the ball at normalised time `t`.
///
/// The ball sweeps left to right over the first half of the cycle and back over
/// the second, bouncing [`BOUNCES`] times each way. `height_factor` is 1 at the
/// bottom of a bounce (fully squashed against the bars) and 0 at the apex.
fn ball_at(t: f64) -> (f64, f64, f64) {
    let x_frac = if t < 0.5 { t / 0.5 } else { (1.0 - t) / 0.5 };
    let index = x_frac * (BARS - 1) as f64;
    let mut bounce_f = (x_frac * BOUNCES) % 1.0;
    if x_frac == 0.0 || x_frac == 1.0 {
        bounce_f = 0.0;
    }
    let bounce_h = 4.0 * bounce_f * (1.0 - bounce_f);
    let height_factor = (1.0 - bounce_h * 2.0).max(0.0);
    (index, bounce_h, height_factor)
}

/// Height of bar `i` and how strongly it is lit, at normalised time `t`.
fn bar_at(i: usize, t: f64) -> (f64, f64) {
    let (index, _, height_factor) = ball_at(t);
    let dist = (i as f64 - index).abs();
    let wave = if dist < 3.0 {
        ((dist / 3.0) * std::f64::consts::FRAC_PI_2).cos()
    } else {
        0.0
    };
    let indent = if dist < 1.5 {
        ((dist / 1.5) * std::f64::consts::FRAC_PI_2).cos() * height_factor * 20.0
    } else {
        0.0
    };
    ((BASE_H + wave * PEAK_H - indent).max(4.0), wave)
}

/// The generated `@keyframes` for the ball and every bar.
///
/// Sampled at [`FRAMES`] points and computed once per process. Consecutive
/// frames that round to the same values are dropped, which cuts the emitted CSS
/// roughly in half — most bars sit at rest for most of the cycle.
pub(crate) fn keyframes() -> &'static str {
    static CACHE: OnceLock<String> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut css = String::from("@keyframes dfx-wave-physics-ball{");
        let mut prev = String::new();
        for k in 0..FRAMES {
            let t = k as f64 / (FRAMES - 1) as f64;
            let (index, bounce_h, height_factor) = ball_at(t);
            let x = index * PITCH;
            let y = (BASE_H + PEAK_H - height_factor * 20.0) + bounce_h * MAX_BOUNCE;
            let body = format!(
                "transform:translate({x:.1}px,{:.1}px) scale({:.3},{:.3})",
                -y,
                1.0 + height_factor * 0.25,
                1.0 - height_factor * 0.3,
            );
            if body != prev || k == FRAMES - 1 {
                css.push_str(&format!("{:.2}%{{{body}}}", t * 100.0));
                prev = body;
            }
        }
        css.push('}');

        for i in 0..BARS {
            css.push_str(&format!("@keyframes dfx-wave-physics-bar-{i}{{"));
            prev.clear();
            for k in 0..FRAMES {
                let t = k as f64 / (FRAMES - 1) as f64;
                let (height, wave) = bar_at(i, t);
                let body = format!(
                    "height:{height:.1}px;background:color-mix(in srgb,var(--dfx-color) {:.0}%,var(--dfx-track))",
                    wave * 100.0
                );
                if body != prev || k == FRAMES - 1 {
                    css.push_str(&format!("{:.2}%{{{body}}}", t * 100.0));
                    prev = body;
                }
            }
            css.push('}');
            css.push_str(&format!(
                ".dfx-wave-physics i:nth-child({}){{animation-name:dfx-wave-physics-bar-{i}}}",
                i + 1
            ));
        }
        css
    })
}

/// A ball bouncing along a row of bars, deforming them as it passes.
///
/// The 201 frames of physics run once at startup and are emitted as CSS
/// keyframes, rather than being recomputed per render, so the animation itself
/// is driven entirely by the compositor.
///
/// The geometry is fixed at 292×192 pixels; use `scale` to fit it into your
/// layout.
#[component]
pub fn WavePhysicsLoader(
    /// Uniform scale applied to the whole loader. `1.0` is 292×192 pixels.
    #[props(default = 1.0)]
    scale: f64,
    /// Colour of the ball and of a bar at the crest of the wave.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Length of one there-and-back pass, in seconds.
    #[props(default = 4.0)]
    duration: f64,
    /// Extra classes for the root element.
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Style { href: "dfx:base", {BASE_CSS} }
        document::Style { href: "dfx:wave-physics-loader", {CSS} }
        document::Style { href: "dfx:wave-physics-loader-keyframes", {keyframes()} }
        div {
            class: "dfx dfx-loader dfx-wave-physics {class}",
            style: "--dfx-scale:{scale};--dfx-color:{color};--dfx-duration:{duration}s;",
            role: "status",
            aria_label: "Loading",
            ..attributes,
            div {
                for n in 0..BARS {
                    i { key: "{n}" }
                }
                b {}
            }
        }
    }
}
