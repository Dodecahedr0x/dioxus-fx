#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod style;

pub use style::{MicroTransitionsStyle, stylesheet};

#[cfg(feature = "loading")]
#[cfg_attr(docsrs, doc(cfg(feature = "loading")))]
pub mod loading;

#[cfg(feature = "text")]
#[cfg_attr(docsrs, doc(cfg(feature = "text")))]
pub mod text;

#[cfg(feature = "entrance")]
#[cfg_attr(docsrs, doc(cfg(feature = "entrance")))]
pub mod entrance;

#[cfg(feature = "hover")]
#[cfg_attr(docsrs, doc(cfg(feature = "hover")))]
pub mod hover;

#[cfg(feature = "cursor")]
#[cfg_attr(docsrs, doc(cfg(feature = "cursor")))]
pub mod cursor;

#[cfg(feature = "scroll")]
#[cfg_attr(docsrs, doc(cfg(feature = "scroll")))]
pub mod scroll;

#[cfg(feature = "buttons")]
#[cfg_attr(docsrs, doc(cfg(feature = "buttons")))]
pub mod buttons;

#[cfg(feature = "cards")]
#[cfg_attr(docsrs, doc(cfg(feature = "cards")))]
pub mod cards;

#[cfg(feature = "primitives")]
#[cfg_attr(docsrs, doc(cfg(feature = "primitives")))]
pub mod primitives;

/// Everything you normally want in scope.
///
/// ```
/// use dioxus_micro_transitions::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{MicroTransitionsStyle, stylesheet};

    #[cfg(feature = "buttons")]
    pub use crate::buttons::*;
    #[cfg(feature = "cards")]
    pub use crate::cards::*;
    #[cfg(feature = "cursor")]
    pub use crate::cursor::*;
    #[cfg(feature = "entrance")]
    pub use crate::entrance::*;
    #[cfg(feature = "hover")]
    pub use crate::hover::*;
    #[cfg(feature = "loading")]
    pub use crate::loading::*;
    // The `AMT_`-prefixed class names, plus `PrimitivesStyle`. Exported as a
    // module too, so `primitives::AMT_ZOOM` reads as well as the bare name.
    #[cfg(feature = "primitives")]
    pub use crate::primitives::{self, *};
    #[cfg(feature = "scroll")]
    pub use crate::scroll::*;
    #[cfg(feature = "text")]
    pub use crate::text::*;
}
