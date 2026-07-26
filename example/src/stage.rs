//! The wrapper every story renders inside, plus the demo content they share.

use dioxus::prelude::*;
use dioxus_micro_transitions::prelude::*;
use dioxus_showcase::prelude::*;

/// Centres each preview and mounts the full stylesheet once.
///
/// Components inject the CSS they need on first use, so this is not required to
/// make them work; mounting [`MicroTransitionsStyle`] up front just keeps the
/// exported static site from flashing unstyled markup as stories mount.
#[provider(index = 0)]
#[component]
pub fn StoryStage(children: Element) -> Element {
    rsx! {
        MicroTransitionsStyle {}
        div {
            style: "display:flex;align-items:center;justify-content:center;\
                    min-height:160px;width:100%;padding:24px;",
            {children}
        }
    }
}

/// A plain block for the entrance and scroll demos to move around.
#[component]
pub fn Swatch() -> Element {
    rsx! {
        div {
            style: "width:72px;height:72px;border-radius:14px;\
                    background:linear-gradient(135deg,#6366f1,#0ea5e9);",
        }
    }
}

/// Inline SVG placeholders, so the gallery pulls in no icon crate.
#[component]
pub fn Chevron() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
            path { d: "M5 12h14M13 6l6 6-6 6" }
        }
    }
}

/// The icon each button swaps to once it has been pressed.
#[component]
pub fn Check() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
            path { d: "M4 12l5 5L20 6" }
        }
    }
}

/// Locally drawn gradients rather than remote photos, so the carousels work
/// offline and in an exported site.
pub fn photos() -> Vec<CardItem> {
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
