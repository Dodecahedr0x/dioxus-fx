//! Stories for `dioxus_micro_transitions::entrance`.
//!
//! Entrance animations run once, when the element mounts, so reload the story
//! or switch away and back to watch one again.

use crate::num;
use crate::stage::Swatch;
use dioxus::prelude::*;
use dioxus_micro_transitions::entrance::*;
use dioxus_showcase::prelude::*;

/// Fades its children in on mount.
#[story(title = "Entrance/FadeIn", tags = ["entrance"])]
pub fn fade_in(duration: f64, delay: f64) -> Element {
    rsx! {
        FadeIn { duration: num(duration, 0.5), delay: num(delay, 0.0),
            Swatch {}
        }
    }
}

/// Fades its children in while they rise into place.
#[story(title = "Entrance/FadeUp", tags = ["entrance"])]
pub fn fade_up(duration: f64, delay: f64, y_offset: f64) -> Element {
    rsx! {
        FadeUp { duration: num(duration, 0.6), delay: num(delay, 0.0), y_offset: num(y_offset, 20.0),
            Swatch {}
        }
    }
}

/// Fades its children in while they descend into place.
#[story(title = "Entrance/FadeDown", tags = ["entrance"])]
pub fn fade_down(duration: f64, delay: f64, y_offset: f64) -> Element {
    rsx! {
        FadeDown { duration: num(duration, 0.6), delay: num(delay, 0.0), y_offset: num(y_offset, -20.0),
            Swatch {}
        }
    }
}

/// Slides its children in from the right, settling leftward.
#[story(title = "Entrance/SlideLeft", tags = ["entrance"])]
pub fn slide_left(duration: f64, delay: f64, x_offset: f64) -> Element {
    rsx! {
        SlideLeft { duration: num(duration, 0.6), delay: num(delay, 0.0), x_offset: num(x_offset, 40.0),
            Swatch {}
        }
    }
}

/// Slides its children in from the left, settling rightward.
#[story(title = "Entrance/SlideRight", tags = ["entrance"])]
pub fn slide_right(duration: f64, delay: f64, x_offset: f64) -> Element {
    rsx! {
        SlideRight { duration: num(duration, 0.6), delay: num(delay, 0.0), x_offset: num(x_offset, -40.0),
            Swatch {}
        }
    }
}

/// Pops its children in with a slight overshoot.
#[story(title = "Entrance/ScaleIn", tags = ["entrance"])]
pub fn scale_in(duration: f64, delay: f64, initial_scale: f64) -> Element {
    rsx! {
        ScaleIn {
            duration: num(duration, 0.5),
            delay: num(delay, 0.0),
            initial_scale: num(initial_scale, 0.92),
            Swatch {}
        }
    }
}

/// Zooms its children in while pulling them out of a blur.
#[story(title = "Entrance/ZoomIn", tags = ["entrance"])]
pub fn zoom_in(duration: f64, delay: f64, initial_scale: f64, initial_blur: String) -> Element {
    rsx! {
        ZoomIn {
            duration: num(duration, 0.7),
            delay: num(delay, 0.0),
            initial_scale: num(initial_scale, 0.85),
            initial_blur: crate::txt(initial_blur, "12px"),
            Swatch {}
        }
    }
}
