# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `surface`: eight effects that layer over markup you already have, behind a
  cargo feature of the same name. `Frost`, `Lens`, `Ripple`, `Peel`, `Vhs`,
  `Glitch`, `Blaze` and `Halftone` each wrap their children in one
  `pointer-events:none` layer, so the content underneath stays selectable,
  clickable and focusable while the effect runs over it. The idea is
  [Canvas UI](https://github.com/DavidHDev/canvas-ui)'s — effects over live
  HTML rather than a canvas that replaced it — reached with `backdrop-filter`,
  `mask` and `mix-blend-mode` instead of WebGL and the experimental
  html-in-canvas API, so there is still no runtime and still no dependency
  beyond `dioxus`. Every one takes an `intensity` from `0` to `1` for dialling
  it down into an existing design, and falls back to a plain translucent layer
  where `backdrop-filter` is unsupported.
- `primitives`: a stylesheet add-on for headless component libraries, behind a
  cargo feature of the same name. `DFX_FADE`, `DFX_ZOOM`, `DFX_SLIDE`,
  `DFX_MENU` and `DFX_COLLAPSE` animate anything that describes its state with
  Radix-style `data-state`/`data-open` attributes — including every component in
  [dioxus-primitives](https://github.com/DioxusLabs/dioxus-components), the
  crate behind <https://dioxuslabs.com/components>. Exit animations included:
  those components keep closing content mounted until its animation finishes.
  The crate gains no dependency; the rules match the attributes, not the crate.

### Added

- The gallery is published to GitHub Pages at <https://dodecahedr0x.github.io/dioxus-fx/>, rebuilt by the `Showcase`
  workflow on every push to `master`. The workflow checks the annotations before
  exporting, and refuses to deploy unless every story title is really present in
  the compiled wasm — a gallery whose registrations were dropped at link time
  builds and deploys perfectly happily while rendering nothing at all.

### Changed

- The gallery builds against
  [dioxus-showcase](https://github.com/Dodecahedr0x/dioxus-showcase) 0.1.1, up
  from 0.0.7. Stories now register themselves at link time rather than through
  generated glue, so `example/showcase/` was rescaffolded rather than migrated:
  its `Cargo.toml` and `src/main.rs` are written once and never rewritten, and
  both carry `lto` in the profile blocks, which is load-bearing — without it the
  wasm linker drops every registration and the gallery launches empty with no
  error. `#[provider(index = 0)]` became `#[provider(order = 0)]`, and a static
  site is now one `dioxus-showcase export` rather than a build plus a `dx
  bundle`. Nothing in the published crate is affected; the gallery lives in
  `example/`.
- Every story parameter now declares what its control opens on, with
  `#[default = …]`, so a control and the preview beside it finally agree. They
  never did: a control opened on `StoryArg`'s placeholder seed — `0`,
  `"Lorem Ipsum"` — while the preview rendered the component's real default, and
  the `num`/`txt` helpers that produced that preview also made `0` and
  `"Lorem Ipsum"` unreachable inputs. Both helpers are gone. The attribute ships
  in dioxus-showcase 0.1.1.
- `cargo test -p gallery` now checks the story registry itself, not just the
  source: that it is non-empty, that every written story reaches it, and that no
  two stories claim one route id. The failure this guards is silent — a story
  that compiles but never registers is indistinguishable from one never written.
- Renamed from `dioxus-micro-transitions` to `dioxus-fx`. "Micro-transitions"
  described the original component set; it does not describe 134 loaders, three
  carousels, a headless-component stylesheet and a set of effects that layer
  over live HTML. The CSS prefix moved with it — classes and custom properties
  are `dfx-`/`--dfx-` rather than `amt-`/`--amt-`, the primitives constants are
  `DFX_FADE` and friends, and `MicroTransitionsStyle` is now `FxStyle`. Nothing
  was published under the old name, so there is no upgrade path to document.
- The gallery is now a
  [dioxus-showcase](https://github.com/Dodecahedr0x/dioxus-showcase) app. Every
  component is annotated with a `#[story]` in the `gallery` crate and the
  browsable site is generated from those annotations — `dioxus-showcase dev`
  replaces `dx serve --package gallery`. Story parameters become live controls,
  so each component can be driven from its own props in the browser.
- The published crate is unchanged: the annotations live in `example/`, so
  `dioxus-fx` still depends on nothing but `dioxus`.

## [0.1.0]

Initial release.

### Added

- 155 components — 134 loading indicators, 7 entrance animations, 4 text
  reveals, 4 hover effects, 3 cursor followers and 3 scroll effects.
- `buttons::AnimatedButton`, covering 12 button interactions, plus
  `buttons::FocusBlurLinks`.
- `cards::CardSpread` with 9 fan layouts, and `cards::CardCarousel`,
  `cards::CardCoverFlow` and `cards::CardTimeMachine` with their `mono` variants.
- Per-module cargo features, all enabled by default.
- `stylesheet()` and `FxStyle` for shipping the CSS yourself instead of
  letting components inject it.

### Notes

- Framer Motion is gone: every animation is CSS keyframes or transitions, and
  the only dependency is `dioxus`.
- Tailwind class props became real CSS colours, so the crate does not require
  Tailwind.
- Springs became the closest cubic-bezier; `WavePhysicsLoader` runs its physics
  once at startup rather than per render; `AppleEqualizer` uses fixed rather than
  random bar delays.

[Unreleased]: https://github.com/Dodecahedr0x/dioxus-fx/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Dodecahedr0x/dioxus-fx/releases/tag/v0.1.0
