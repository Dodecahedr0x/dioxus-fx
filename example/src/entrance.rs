//! Stories for `dioxus_fx::entrance`.
//!
//! Entrance animations run once, when the element mounts, so reload the story
//! or switch away and back to watch one again.

use crate::stage::Swatch;
use dioxus::prelude::*;
use dioxus_fx::entrance::*;
use dioxus_showcase::prelude::*;

/// Fades its children in on mount.
#[story(title = "Entrance/FadeIn", tags = ["entrance"])]
pub fn fade_in(#[default = 0.5] duration: f64, #[default = 0.0] delay: f64) -> Element {
    rsx! {
        FadeIn { duration: duration, delay: delay,
            Swatch {}
        }
    }
}

/// Fades its children in while they rise into place.
#[story(title = "Entrance/FadeUp", tags = ["entrance"])]
pub fn fade_up(
    #[default = 0.6] duration: f64,
    #[default = 0.0] delay: f64,
    #[default = 20.0] y_offset: f64,
) -> Element {
    rsx! {
        FadeUp { duration: duration, delay: delay, y_offset: y_offset,
            Swatch {}
        }
    }
}

/// Fades its children in while they descend into place.
#[story(title = "Entrance/FadeDown", tags = ["entrance"])]
pub fn fade_down(
    #[default = 0.6] duration: f64,
    #[default = 0.0] delay: f64,
    #[default = -20.0] y_offset: f64,
) -> Element {
    rsx! {
        FadeDown { duration: duration, delay: delay, y_offset: y_offset,
            Swatch {}
        }
    }
}

/// Slides its children in from the right, settling leftward.
#[story(title = "Entrance/SlideLeft", tags = ["entrance"])]
pub fn slide_left(
    #[default = 0.6] duration: f64,
    #[default = 0.0] delay: f64,
    #[default = 40.0] x_offset: f64,
) -> Element {
    rsx! {
        SlideLeft { duration: duration, delay: delay, x_offset: x_offset,
            Swatch {}
        }
    }
}

/// Slides its children in from the left, settling rightward.
#[story(title = "Entrance/SlideRight", tags = ["entrance"])]
pub fn slide_right(
    #[default = 0.6] duration: f64,
    #[default = 0.0] delay: f64,
    #[default = -40.0] x_offset: f64,
) -> Element {
    rsx! {
        SlideRight { duration: duration, delay: delay, x_offset: x_offset,
            Swatch {}
        }
    }
}

/// Pops its children in with a slight overshoot.
#[story(title = "Entrance/ScaleIn", tags = ["entrance"])]
pub fn scale_in(
    #[default = 0.5] duration: f64,
    #[default = 0.0] delay: f64,
    #[default = 0.92] initial_scale: f64,
) -> Element {
    rsx! {
        ScaleIn {
            duration,
            delay,
            initial_scale,
            Swatch {}
        }
    }
}

/// Zooms its children in while pulling them out of a blur.
#[story(title = "Entrance/ZoomIn", tags = ["entrance"])]
pub fn zoom_in(
    #[default = 0.7] duration: f64,
    #[default = 0.0] delay: f64,
    #[default = 0.85] initial_scale: f64,
    #[default = "12px"] initial_blur: String,
) -> Element {
    rsx! {
        ZoomIn {
            duration,
            delay,
            initial_scale,
            initial_blur,
            Swatch {}
        }
    }
}
