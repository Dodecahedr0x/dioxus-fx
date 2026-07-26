# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0]

Initial release: a Dioxus port of
[Amicro](https://github.com/Subhan-code/Amicro--Micro-transitions-).

### Added

- All 155 components from the Amicro registry — 134 loading indicators, 7
  entrance animations, 4 text reveals, 4 hover effects, 3 cursor followers and 3
  scroll effects.
- `buttons::AnimatedButton`, covering the 12 upstream button interactions, plus
  `buttons::FocusBlurLinks`.
- `cards::CardSpread` with the 9 upstream fan layouts, and `cards::CardCarousel`,
  `cards::CardCoverFlow` and `cards::CardTimeMachine` with their `mono` variants.
- Per-module cargo features, all enabled by default.
- `stylesheet()` and `MicroTransitionsStyle` for shipping the CSS yourself
  instead of letting components inject it.

### Notes

- Framer Motion is gone: every animation is CSS keyframes or transitions, and
  the only dependency is `dioxus`.
- Tailwind class props became real CSS colours, so the crate does not require
  Tailwind.
- Springs became the closest cubic-bezier; `WavePhysicsLoader` runs its physics
  once at startup rather than per render; `AppleEqualizer` uses fixed rather than
  random bar delays.

[Unreleased]: https://github.com/dode/dioxus-micro-transitions/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/dode/dioxus-micro-transitions/releases/tag/v0.1.0
