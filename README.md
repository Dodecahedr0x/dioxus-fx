# dioxus-fx

[![crates.io](https://img.shields.io/crates/v/dioxus-fx.svg)](https://crates.io/crates/dioxus-fx)
[![docs.rs](https://img.shields.io/docsrs/dioxus-fx)](https://docs.rs/dioxus-fx)
[![CI](https://github.com/Dodecahedr0x/dioxus-fx/actions/workflows/ci.yml/badge.svg)](https://github.com/Dodecahedr0x/dioxus-fx/actions/workflows/ci.yml)
[![licence](https://img.shields.io/crates/l/dioxus-fx.svg)](#licence)
[![gallery](https://img.shields.io/badge/gallery-live-8b5cf6)](https://dodecahedr0x.github.io/dioxus-fx/)

Visual effects and animated components for [Dioxus](https://dioxuslabs.com):
loaders, entrances, text reveals, pointer effects, and effects that layer over
markup you already have. All of it plain CSS — no animation runtime, no CSS
framework, one dependency.

```toml
[dependencies]
dioxus = "0.7"
dioxus-fx = "0.1"
```

```rust
use dioxus::prelude::*;
use dioxus_fx::prelude::*;

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

185 components, in ten modules, each behind its own cargo feature.

| Module | Count | What's in it |
| --- | --- | --- |
| `loading` | 134 | Spinners, progress rings, skeletons, activity indicators |
| `entrance` | 7 | Mount-time reveals — fade, slide, scale, zoom |
| `text` | 4 | Staggered reveals by character, word or line |
| `hover` | 4 | Glow button, magnetic button, tilt card, card grid |
| `cursor` | 3 | Spotlight, cursor ring, cursor trail |
| `surface` | 24 | Effects that layer over markup you already have |
| `scroll` | 3 | Scroll progress bar, scroll reveal, sticky reader |
| `buttons` | 12 + 1 | Button interactions behind one component, plus a focus-blur link row |
| `cards` | 9 + 3 | Hover-fanned card layouts and three carousels |
| `primitives` | 5 | Enter/exit animations for headless component libraries |

## Effects over live HTML

[Canvas UI](https://github.com/DavidHDev/canvas-ui) makes a good argument: the
interesting place for a visual effect is *over* live HTML — text still
selectable, links still clickable — not inside a canvas that has replaced it. It
gets there with WebGL and the experimental html-in-canvas API. The `surface`
module takes the same idea to where CSS already reaches it: `backdrop-filter`
reads whatever is painted behind an element, `mask` and `mix-blend-mode` shape
the result, and the content underneath never stops being ordinary DOM.

```rust
use dioxus::prelude::*;
use dioxus_fx::surface::*;

fn Paywalled(article: Element) -> Element {
    rsx! {
        // Frozen over, clear wherever the pointer goes. Still selectable.
        Frost { melt: 160.0, {article} }
    }
}
```

| Component | What it does | After Canvas UI's |
| --- | --- | --- |
| `Frost` | A frozen pane that melts clear around the pointer | `Frost` |
| `Lens` | A glass puck that follows the pointer and sharpens what is under it | `Glass`, `Magnify` |
| `Ripple` | Rings that spread from every click and bend the content they cross | `Ripple` |
| `Peel` | A corner that lifts on hover, showing a second layer | `Peel` |
| `Vhs` | Worn tape: scanlines, chroma bleed, head noise, grain | `VHS` |
| `Glitch` | Broadcast tearing bursts, idle in between | `Glitch` |
| `Blaze` | Embers and heat haze rising over the content | `Blaze` |
| `Halftone` | A retro dither screen — the one that never animates | `Retro Dither` |
| `Bubble` | Soap bubbles trailing the pointer, refracting the page | `Bubble` |
| `Mist` | Drifting banks of mist, parted by the pointer | `Clouds` |
| `Droplets` | Rain running down, refracting whatever it crosses | `Droplets` |
| `Tiles` | Lit tiles rippling in a diagonal wave around the pointer | `Grid` |
| `Honeycomb` | Floating hex tiles that shine under the pointer | `Hex Float` |
| `Laser` | A beam that scans down, revealing the content behind it | `Laser` |
| `Shatter` | Glass that breaks wherever the content is clicked | `Shatter` |
| `Stipple` | Fine grain that resolves back into crisp UI at the pointer | `Particle Reveal` |
| `Liquid` | A wash of fluid the pointer drags through the content | `Liquid` |
| `Dissolve` | Content below a line dissolved into grains, reassembled on scroll | `Particle Scroll` |
| `Bend` | A block that folds away over a virtual edge as the page scrolls | `Bend` |
| `Ascii` | A lens leaving the page visible only through glyph-shaped holes | `Asciify` |
| `Cloth` | Content hung on fabric that breathes, swelling under the pointer | `Cloth` |

Each wraps its children in one `pointer-events:none` layer, so dropping it
around a section changes how that section looks and nothing else: no canvas, no
shader, no duplicated markup, no new dependency. Each takes an `intensity`
between `0` and `1`, because the version of an effect a real design can live
with is usually the same effect turned most of the way down. Where
`backdrop-filter` is unsupported they fall back to a plain translucent layer,
which is dimmer but never blank.

Three more are objects rather than wrappers. They take a `src` — an SVG or an
image whose alpha is the shape — and fill that outline with the page behind it,
which is the same split Canvas UI draws between its components and its
`… Object` ones.

| Component | What it does | After Canvas UI's |
| --- | --- | --- |
| `GlassShape` | A silhouette turned into floating glass | `Glass Object` |
| `DitherShape` | A silhouette screened through a one-bit dither | `Dithered Object` |
| `ParticleShape` | A silhouette rebuilt as particles that scatter on hover | `Particle Object` |

That is all 25 of Canvas UI's components — in spirit, not in method. WebGL can
do things CSS cannot, and where the mechanism differs the component says so:
`Lens` resolves where `Magnify` enlarges, `Bubble` overlaps where Canvas UI's
merges as metaballs, `Shatter` stays in plane rather than lifting each shard
into 3D, `Bend` folds a section where `Bend` folds a scanline, and the three
shape components take the flat inputs only — an SVG, a PNG, an icon — where
Canvas UI's also accept a GLB model.

The last two are the biggest stretch and are labelled as such. Real ASCII art
picks the character matching each cell's brightness, which means reading the
rendered pixels back; `Ascii` instead punches a fixed glyph field out of a sheet
and lets the page show through the holes, which is closer to `Halftone`'s fixed
dot grid than to a converter. `Cloth` simulates nothing: the fabric is shading,
the wind is one long keyframe, and the pointer lights the cloth rather than
pushing it. Both borrow the read, not the physics.

## With dioxus-primitives

[`dioxus-primitives`](https://github.com/DioxusLabs/dioxus-components) — the
crate behind <https://dioxuslabs.com/components> — ships unstyled components
that describe their state with data attributes: `data-state="open"`,
`data-open="true"`, `data-side`, `data-align`. The `primitives` module is a
stylesheet keyed on exactly those, so animating one is adding a class.

```rust
use dioxus::prelude::*;
use dioxus_fx::primitives::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

fn Modal(open: bool) -> Element {
    rsx! {
        PrimitivesStyle {}
        DialogRoot { open, class: "backdrop {DFX_FADE}",
            DialogContent { class: "panel {DFX_ZOOM}", "Hello" }
        }
    }
}
```

Enter *and* exit: the primitives hold closing content in the DOM until its
animation finishes, so both halves play. Timing is per-element through
`--dfx-enter` and `--dfx-exit`, the keyframes animate `translate`/`scale`
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
dioxus-fx = { version = "0.1", default-features = false, features = ["loading"] }
```

## Gallery

**<https://dodecahedr0x.github.io/dioxus-fx/>** — every component, live, with its props as controls.

The gallery is a [dioxus-showcase](https://github.com/Dodecahedr0x/dioxus-showcase)
app: every component is annotated with a `#[story]` in the `gallery` crate
(`example/`), and the browsable site is generated from those annotations. Pushing
to `master` republishes it through the `Showcase` workflow.

```sh
cargo install dioxus-showcase-cli --locked   # provides `dioxus-showcase`
cargo install dioxus-cli --locked            # provides `dx`
rustup target add wasm32-unknown-unknown

dioxus-showcase check    # validate the annotations
dioxus-showcase dev      # live gallery at http://127.0.0.1:6111
```

For a deployable static site:

```sh
dioxus-showcase export                          # → target/showcase/site
dioxus-showcase export --base-path /dioxus-fx   # as published, under a sub-path
```

`--base-path` has to match the sub-path the site is served from, or every asset
URL and router link resolves against the wrong root. GitHub Pages serves this
repo from `/dioxus-fx`, which is what the workflow passes.

Every component, live, with its props as controls: change a spinner's size,
colour or duration and the preview follows. Leave a control untouched and the
story renders the component's documented default.

Adding a component means adding its story — `cargo test -p gallery` fails on any
component that has none, and on any story that compiles but never registers.

## Documentation

Full API docs, per-component notes and browser-support caveats are on
[docs.rs](https://docs.rs/dioxus-fx).

## Contributing

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

Components live one-per-file under `dioxus-fx/src/loading/`; the smaller
categories are single modules. A component owns its CSS as a `pub(crate) const
CSS`, and the tests cross-check that every class the markup renders has a rule
behind it.

## Licence

MIT or Apache-2.0, at your option.

The loading, entrance, text, hover, cursor, scroll, button and card components
began as a port of [Amicro](https://github.com/Subhan-code/Amicro--Micro-transitions-),
which is MIT-licensed.
