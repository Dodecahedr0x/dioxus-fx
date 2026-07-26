# dioxus-micro-transitions

[![crates.io](https://img.shields.io/crates/v/dioxus-micro-transitions.svg)](https://crates.io/crates/dioxus-micro-transitions)
[![docs.rs](https://img.shields.io/docsrs/dioxus-micro-transitions)](https://docs.rs/dioxus-micro-transitions)
[![CI](https://github.com/dode/dioxus-micro-transitions/actions/workflows/ci.yml/badge.svg)](https://github.com/dode/dioxus-micro-transitions/actions/workflows/ci.yml)
[![licence](https://img.shields.io/crates/l/dioxus-micro-transitions.svg)](#licence)

Micro-interactions and transition components for [Dioxus](https://dioxuslabs.com) —
a port of [Amicro](https://github.com/Subhan-code/Amicro--Micro-transitions-),
rebuilt without the JavaScript animation runtime.

```toml
[dependencies]
dioxus = "0.7"
dioxus-micro-transitions = "0.1"
```

```rust
use dioxus::prelude::*;
use dioxus_micro_transitions::prelude::*;

fn App() -> Element {
    rsx! {
        FadeUp { delay: 0.1,
            h1 { "Dashboard" }
        }
        IosSpinner { size: 24.0, color: "#3b82f6" }
        CardSpread { layout: CardSpreadLayout::Arc5 }
    }
}
```

Nothing to configure: each component injects the CSS it needs on first use.

## What's in it

All 155 components from the Amicro registry, plus its button and card
interaction sets.

| Module | Count | What's in it |
| --- | --- | --- |
| `loading` | 134 | Spinners, progress rings, skeletons, activity indicators |
| `entrance` | 7 | Mount-time reveals — fade, slide, scale, zoom |
| `text` | 4 | Staggered reveals by character, word or line |
| `hover` | 4 | Glow button, magnetic button, tilt card, card grid |
| `cursor` | 3 | Spotlight, cursor ring, cursor trail |
| `scroll` | 3 | Scroll progress bar, scroll reveal, sticky reader |
| `buttons` | 12 + 1 | Button interactions behind one component, plus a focus-blur link row |
| `cards` | 9 + 3 | Hover-fanned card layouts and three carousels |
| `primitives` | 5 | Enter/exit animations for headless component libraries |

## With dioxus-primitives

[`dioxus-primitives`](https://github.com/DioxusLabs/dioxus-components) — the
crate behind <https://dioxuslabs.com/components> — ships unstyled components
that describe their state with data attributes: `data-state="open"`,
`data-open="true"`, `data-side`, `data-align`. The `primitives` module is a
stylesheet keyed on exactly those, so animating one is adding a class.

```rust
use dioxus::prelude::*;
use dioxus_micro_transitions::primitives::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

fn Modal(open: bool) -> Element {
    rsx! {
        PrimitivesStyle {}
        DialogRoot { open, class: "backdrop {AMT_FADE}",
            DialogContent { class: "panel {AMT_ZOOM}", "Hello" }
        }
    }
}
```

Enter *and* exit: the primitives hold closing content in the DOM until its
animation finishes, so both halves play. Timing is per-element through
`--amt-enter` and `--amt-exit`, the keyframes animate `translate`/`scale`
rather than the `transform` shorthand so they compose with the library's own
positioning, and `prefers-reduced-motion` shortens them to nothing rather than
removing them.

Nothing in this crate depends on `dioxus-primitives`; the rules match those
attributes wherever they come from.

## Why it's small

- **One dependency.** Just `dioxus`. No Framer Motion, no Tailwind, no CSS
  framework.
- **Compositor-driven.** Every animation is CSS keyframes or transitions, so
  none of it runs on the main thread.
- **Feature-gated.** Each module is a cargo feature. Drop the ones you do not
  use and their CSS goes with them.

```toml
dioxus-micro-transitions = { version = "0.1", default-features = false, features = ["loading"] }
```

## Gallery

The gallery is a [dioxus-showcase](https://github.com/Dodecahedr0x/dioxus-showcase)
app: every component is annotated with a `#[story]` in the `gallery` crate
(`example/`), and the browsable site is generated from those annotations.

```sh
cargo install dioxus-showcase-cli --locked   # provides `dioxus-showcase`
cargo install dioxus-cli --locked            # provides `dx`
rustup target add wasm32-unknown-unknown

dioxus-showcase check    # validate the annotations
dioxus-showcase dev      # live gallery at http://127.0.0.1:6111
```

For a static site, regenerate the app and bundle it:

```sh
dioxus-showcase build
(cd example/showcase && dx bundle --platform web --release)   # → example/showcase/dist/public
```

Every component, live, with its props as controls: change a spinner's size,
colour or duration and the preview follows. Leave a control untouched and the
story renders the component's documented default.

Adding a component means adding its story — `cargo test -p gallery` fails on any
component that has none.

## Documentation

Full API docs, per-component notes, browser-support caveats and the list of
deliberate differences from upstream are on
[docs.rs](https://docs.rs/dioxus-micro-transitions).

## Contributing

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

Components live one-per-file under `dioxus-micro-transitions/src/loading/`; the
smaller categories are single modules. A component owns its CSS as a
`pub(crate) const CSS`, and the tests cross-check that every class the markup
renders has a rule behind it.

## Licence

MIT or Apache-2.0, at your option. Upstream Amicro is MIT-licensed.
