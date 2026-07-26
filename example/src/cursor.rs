//! Stories for `dioxus_fx::cursor`.
//!
//! `MouseFollow` and `CursorTrail` are page-wide effects: they track the
//! pointer anywhere in the showcase window, not only over the preview.

use dioxus::prelude::*;
use dioxus_fx::cursor::*;
use dioxus_showcase::prelude::*;

/// A panel lit by a soft glow that follows the pointer across it.
#[story(title = "Cursor/Spotlight", tags = ["cursor"])]
pub fn spotlight(
    #[default = "rgba(255,255,255,.08)"] glow_color: String,
    #[default = 250.0] glow_size: f64,
) -> Element {
    rsx! {
        Spotlight {
            glow_color,
            glow_size,
            div { style: "padding:48px 32px;font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
                "Move the pointer across this panel."
            }
        }
    }
}

/// A dot that chases the pointer with a spring-like lag.
#[story(title = "Cursor/MouseFollow", tags = ["cursor"])]
pub fn mouse_follow(#[default = 0.35] lag: f64, #[default = "#3b82f6"] color: String) -> Element {
    rsx! {
        MouseFollow { lag: lag, color: color,
            div { style: "padding:48px 32px;font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
                "The ring trails the pointer."
            }
        }
    }
}

/// A string of dots trailing the pointer, each lagging further behind.
#[story(title = "Cursor/CursorTrail", tags = ["cursor"])]
pub fn cursor_trail(
    #[default = 8.0] size: f64,
    #[default = 6] count: usize,
    #[default = "#3b82f6"] color: String,
    #[default = 0.12] lag: f64,
) -> Element {
    rsx! {
        CursorTrail {
            size,
            count,
            color,
            lag,
        }
        div { style: "font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
            "Move the pointer anywhere on the page."
        }
    }
}
