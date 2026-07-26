//! Stylesheet plumbing.
//!
//! Every component in this crate is animated with plain CSS keyframes — there is
//! no JavaScript animation runtime and no CSS framework dependency. Each
//! component injects the small stylesheet it needs into `<head>` on first use,
//! deduplicated by key, so in the common case you do not have to do anything.
//!
//! If you prefer to ship the CSS yourself (static site export, a strict CSP that
//! forbids injected `<style>` tags, or simply to avoid a flash of unstyled
//! content), use [`stylesheet`] to obtain the full CSS as a string, or mount
//! [`MicroTransitionsStyle`] once near the root of your app.

use dioxus::prelude::*;

/// CSS shared by every component: the custom-property defaults that the
/// per-component rules read, plus the reduced-motion escape hatch.
pub(crate) const BASE_CSS: &str = r#"
.amt{--amt-size:40px;--amt-color:currentColor;--amt-duration:1s;--amt-track:color-mix(in srgb,currentColor 15%,transparent);box-sizing:border-box}
.amt *{box-sizing:border-box}
@supports not (color: color-mix(in srgb,red 50%,transparent)){.amt{--amt-track:rgba(128,128,128,.25)}}
@media (prefers-reduced-motion:reduce){
.amt-decorative,.amt-decorative *{animation:none!important;transition-duration:.01ms!important}
.amt-loader,.amt-loader *{animation-duration:calc(var(--amt-duration)*3)!important}
}
"#;

/// Render a `<style>` element into `<head>` exactly once per `key`.
///
/// Dioxus deduplicates head styles by their `href`, so repeated renders of the
/// same component — or many instances of it — only ever produce one `<style>`
/// tag. Returns an `Element` meant to be embedded in a component's `rsx!`.
// Every caller lives behind a feature, so with all of them off this is dead —
// which is a configuration to compile cleanly, not to warn about.
#[allow(unused_macros)]
macro_rules! amt_style {
    ($key:literal, $css:expr) => {
        rsx! {
            ::dioxus::prelude::document::Style { href: "amt:base", {crate::style::BASE_CSS} }
            ::dioxus::prelude::document::Style { href: concat!("amt:", $key), {$css} }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use amt_style;

/// Every stylesheet fragment this build contains, in a stable order.
///
/// Fragments already include [`BASE_CSS`], which is why [`stylesheet`]
/// concatenates the base once and then only the category bodies.
fn fragments() -> Vec<&'static str> {
    // Nothing extends `parts` when every module feature is off.
    #[allow(unused_mut)]
    let mut parts = vec![BASE_CSS];
    #[cfg(feature = "loading")]
    parts.extend_from_slice(crate::loading::CSS);
    #[cfg(feature = "text")]
    parts.extend_from_slice(crate::text::CSS);
    #[cfg(feature = "entrance")]
    parts.extend_from_slice(crate::entrance::CSS);
    #[cfg(feature = "hover")]
    parts.extend_from_slice(crate::hover::CSS);
    #[cfg(feature = "cursor")]
    parts.extend_from_slice(crate::cursor::CSS);
    #[cfg(feature = "scroll")]
    parts.extend_from_slice(crate::scroll::CSS);
    #[cfg(feature = "buttons")]
    parts.extend_from_slice(crate::buttons::CSS);
    #[cfg(feature = "cards")]
    parts.extend_from_slice(crate::cards::CSS);
    #[cfg(feature = "primitives")]
    parts.extend_from_slice(crate::primitives::CSS);
    parts
}

/// The complete stylesheet for every component enabled by the active cargo
/// features.
///
/// Useful for writing a `.css` file at build time, or for inlining the CSS into
/// a template yourself:
///
/// ```
/// let css = dioxus_micro_transitions::stylesheet();
/// assert!(css.contains("@keyframes"));
/// ```
pub fn stylesheet() -> String {
    // Only the `loading` feature appends to it.
    #[allow(unused_mut)]
    let mut css = fragments().concat();
    // One component computes its keyframes rather than declaring them, so it
    // cannot contribute a `&'static str` fragment up front.
    #[cfg(feature = "loading")]
    css.push_str(crate::loading::wave_physics_keyframes());
    css
}

/// Mounts the crate's full stylesheet into `<head>`.
///
/// Optional — components inject what they need on their own. Mount this once at
/// the root of your app if you would rather have all the CSS present up front.
///
/// ```rust, no_run
/// use dioxus::prelude::*;
/// use dioxus_micro_transitions::MicroTransitionsStyle;
///
/// fn App() -> Element {
///     rsx! {
///         MicroTransitionsStyle {}
///         // ... the rest of your app
///     }
/// }
/// ```
#[component]
pub fn MicroTransitionsStyle() -> Element {
    // `document::Style` needs a single text node, so the whole sheet goes in as
    // one owned string built once per app.
    let css = use_hook(stylesheet);
    rsx! {
        document::Style { href: "amt:all", {css} }
    }
}
