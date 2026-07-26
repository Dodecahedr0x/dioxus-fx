//! Stories for `dioxus_fx::cursor`.
//!
//! `MouseFollow` and `CursorTrail` are page-wide effects: they track the
//! pointer anywhere in the showcase window, not only over the preview.

use crate::{num, txt};
use dioxus::prelude::*;
use dioxus_fx::cursor::*;
use dioxus_showcase::prelude::*;

/// A panel lit by a soft glow that follows the pointer across it.
#[story(title = "Cursor/Spotlight", tags = ["cursor"])]
pub fn spotlight(glow_color: String, glow_size: f64) -> Element {
    rsx! {
        Spotlight {
            glow_color: txt(glow_color, "rgba(255,255,255,.08)"),
            glow_size: num(glow_size, 250.0),
            div { style: "padding:48px 32px;font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
                "Move the pointer across this panel."
            }
        }
    }
}

/// A dot that chases the pointer with a spring-like lag.
#[story(title = "Cursor/MouseFollow", tags = ["cursor"])]
pub fn mouse_follow(lag: f64, color: String) -> Element {
    rsx! {
        MouseFollow { lag: num(lag, 0.35), color: txt(color, "#3b82f6"),
            div { style: "padding:48px 32px;font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
                "The ring trails the pointer."
            }
        }
    }
}

/// A string of dots trailing the pointer, each lagging further behind.
#[story(title = "Cursor/CursorTrail", tags = ["cursor"])]
pub fn cursor_trail(size: f64, count: usize, color: String, lag: f64) -> Element {
    rsx! {
        CursorTrail {
            size: num(size, 8.0),
            count: num(count, 6),
            color: txt(color, "#3b82f6"),
            lag: num(lag, 0.12),
        }
        div { style: "font:500 15px/1.4 ui-sans-serif,system-ui,sans-serif;",
            "Move the pointer anywhere on the page."
        }
    }
}
