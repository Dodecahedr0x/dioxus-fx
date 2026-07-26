//! Showcase stories for every component in `dioxus-fx`.
//!
//! This crate is the entry crate for [`dioxus-showcase`](https://github.com/Dodecahedr0x/dioxus-showcase):
//! every component in the library has a `#[story]` here, so the browsable
//! gallery is generated from annotations rather than hand-written pages.
//!
//! ```bash
//! dioxus-showcase dev      # live gallery at http://127.0.0.1:6111
//! dioxus-showcase export   # static site in target/showcase/site
//! ```
//!
//! Story parameters become live controls in the showcase shell, and each one
//! carries `#[default = …]` naming the value its control opens on — the same
//! value the component itself documents. Without that attribute a control opens
//! on `StoryArg`'s placeholder seed (`0`, `"Lorem Ipsum"`), which is a value the
//! preview is not rendering.

pub mod buttons;
pub mod cards;
pub mod cursor;
pub mod entrance;
pub mod hover;
pub mod loading;
pub mod primitives;
pub mod scroll;
pub mod stage;
pub mod surface;
pub mod text;
