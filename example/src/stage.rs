//! The wrapper every story renders inside, plus the demo content they share.

use dioxus::prelude::*;
use dioxus_fx::prelude::*;
use dioxus_showcase::prelude::*;

/// Centres each preview and mounts the full stylesheet once.
///
/// Components inject the CSS they need on first use, so this is not required to
/// make them work; mounting [`FxStyle`] up front just keeps the exported
/// static site from flashing unstyled markup as stories mount.
#[provider(order = 0)]
#[component]
pub fn StoryStage(children: Element) -> Element {
    rsx! {
        FxStyle {}
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

/// The slides the carousel stories share: a local placeholder, a caption and a
/// timestamp each.
///
/// The artwork is a stand-in. To use real images, drop them into
/// `example/assets/photos/` and point these paths at them — `asset!` resolves at
/// compile time, so the extension is part of the path and has to match the file
/// on disk.
const PHOTOS: [(Asset, &str, &str); 5] = [
    (asset!("/assets/photos/photo-1.svg"), "Sunset", "Today"),
    (asset!("/assets/photos/photo-2.svg"), "Dusk", "1d ago"),
    (asset!("/assets/photos/photo-3.svg"), "Forest", "1w ago"),
    (asset!("/assets/photos/photo-4.svg"), "Sunlight", "1m ago"),
    (asset!("/assets/photos/photo-5.svg"), "Hills", "1y ago"),
];

/// Bundled placeholders rather than remote photos, so the carousels work
/// offline and in an exported site.
pub fn photos() -> Vec<CardItem> {
    PHOTOS
        .iter()
        .map(|(src, title, date)| CardItem::new(src.to_string(), *title).with_date(*date))
        .collect()
}
