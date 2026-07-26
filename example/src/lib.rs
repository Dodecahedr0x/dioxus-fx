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
//! Story parameters become live controls in the showcase shell. They are seeded
//! from [`dioxus_showcase::StoryArg`], which hands back `0` for numbers and
//! `"Lorem Ipsum"` for strings, so every story runs its seeds through [`num`]
//! and [`txt`] to open on the component's documented default instead.

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

/// Returns `fallback` while a numeric control still holds its zero seed.
///
/// Zero is not a useful value for any prop in this library — a spinner sized
/// `0px` is invisible — so it doubles as "untouched" without costing a real
/// setting. Negative values pass straight through, which the entrance
/// components need.
pub fn num<T: Default + PartialEq>(value: T, fallback: T) -> T {
    if value == T::default() {
        fallback
    } else {
        value
    }
}

/// Returns `fallback` while a text control still holds its placeholder seed.
pub fn txt(value: String, fallback: &str) -> String {
    if value.is_empty() || value == "Lorem Ipsum" {
        fallback.to_string()
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::{num, txt};

    #[test]
    fn num_replaces_only_the_zero_seed() {
        assert_eq!(num(0.0, 32.0), 32.0);
        assert_eq!(num(12.0, 32.0), 12.0);
        assert_eq!(num(-20.0, 40.0), -20.0);
        assert_eq!(num(0usize, 6), 6);
    }

    #[test]
    fn txt_replaces_placeholder_and_empty_seeds() {
        assert_eq!(txt(String::new(), "currentColor"), "currentColor");
        assert_eq!(txt("Lorem Ipsum".into(), "currentColor"), "currentColor");
        assert_eq!(txt("#f97316".into(), "currentColor"), "#f97316");
    }
}
