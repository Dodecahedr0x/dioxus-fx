# dioxus-fx

Visual effects and animated components for [Dioxus](https://dioxuslabs.com):
loaders, entrances, text reveals, pointer effects, and effects that layer over
markup you already have. 169 components, in ten modules, each behind its own
cargo feature.

| Module | What's in it |
| --- | --- |
| [`loading`] | 134 spinners, progress rings, skeletons and activity indicators |
| [`entrance`] | 7 mount-time reveals — fade, slide, scale, zoom |
| [`text`] | 4 staggered text reveals — by character, word or line |
| [`hover`] | 4 pointer-reactive pieces — glow, magnetic, tilt, card grid |
| [`cursor`] | 3 cursor followers — spotlight, ring, trail |
| [`surface`] | 8 effects that layer over markup you already have |
| [`scroll`] | 3 scroll effects — progress bar, reveal, sticky reader |
| [`buttons`] | 12 button interactions behind one component, plus a focus-blur link row |
| [`cards`] | 9 hover-fanned card layouts and 3 carousels |
| [`primitives`] | Enter/exit animations for headless component libraries |

## Design

**No animation runtime.** No JavaScript animation library, no CSS framework,
and exactly one dependency — `dioxus` itself. Every animation is CSS keyframes
or transitions, so it runs on the compositor rather than the main thread.

**Nothing to set up.** Each component injects the small stylesheet it needs into
`<head>` on first use, deduplicated by key. Use a component and it works. If you
would rather ship the CSS yourself, [`stylesheet`] hands you the whole thing
as a string and [`FxStyle`] mounts it in one place.

**Consistent props.** Every loader takes `size` in pixels, `color` as any CSS
colour (defaulting to `currentColor`, so it inherits), and `duration` in
seconds. Everything takes `class` and forwards arbitrary attributes, so `id`,
`data-*`, `aria-*` and event handlers all pass through.

**Accessible by default.** Loaders render `role="status"` with a label.
Decorative motion honours `prefers-reduced-motion`; loaders slow down rather
than freeze, so they still read as "working".

## Install

```toml
[dependencies]
dioxus = "0.7"
dioxus-fx = "0.1"
```

## Use

```rust, no_run
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

Colours default to `currentColor`, so a loader picks up the text colour around
it:

```rust, no_run
# use dioxus::prelude::*;
# use dioxus_fx::prelude::*;
fn Row() -> Element {
    rsx! {
        span { style: "color: tomato",
            RingSweep { size: 16.0 }
        }
    }
}
```

## Features

Each module is a cargo feature, all on by default. Turn off what you do not use
and its CSS leaves the binary with it:

```toml
[dependencies]
dioxus-fx = { version = "0.1", default-features = false, features = ["loading"] }
```

`loading`, `entrance`, `text`, `hover`, `cursor`, `surface`, `scroll`,
`buttons`, `cards`, `primitives`.

## Effects over live HTML

The [`surface`] module wraps markup you already have and paints one
`pointer-events:none` layer over it, so the content underneath stays selectable,
clickable and focusable while the effect runs. `backdrop-filter` reads whatever
is painted behind the layer, `mask` and `mix-blend-mode` shape the result:

```rust, no_run
# use dioxus::prelude::*;
use dioxus_fx::surface::*;

fn Paywalled(article: Element) -> Element {
    rsx! {
        // Frozen over, clear wherever the pointer goes. Still selectable.
        Frost { melt: 160.0, {article} }
    }
}
```

[`Frost`](surface::Frost), [`Lens`](surface::Lens), [`Ripple`](surface::Ripple),
[`Peel`](surface::Peel), [`Vhs`](surface::Vhs), [`Glitch`](surface::Glitch),
[`Blaze`](surface::Blaze) and [`Halftone`](surface::Halftone) each take an
`intensity` from `0` to `1`, because the version of an effect a real design can
live with is usually the same effect turned most of the way down. The idea is
[Canvas UI](https://github.com/DavidHDev/canvas-ui)'s; this is the CSS-native
route to it, so there is still no WebGL and still no dependency.

## With dioxus-primitives

[`dioxus-primitives`](https://github.com/DioxusLabs/dioxus-components) — the
crate behind <https://dioxuslabs.com/components> — ships unstyled components
that describe their state with data attributes. The [`primitives`] module is a
stylesheet keyed on those attributes, so animating one is adding a class:

```rust, no_run
# use dioxus::prelude::*;
use dioxus_fx::primitives::*;

fn App() -> Element {
    rsx! {
        PrimitivesStyle {}
        // DialogRoot   { class: "dx-dialog-backdrop {DFX_FADE}", .. }
        // DialogContent{ class: "dx-dialog {DFX_ZOOM}", .. }
        // PopoverContent { class: "{DFX_SLIDE}", .. }   // follows data-side
        // AccordionContent { class: "{DFX_COLLAPSE}", .. }
    }
}
```

Enter *and* exit: the primitives hold closing content in the DOM until its
animation finishes. Nothing here depends on `dioxus-primitives` — the rules
match `data-state`, `data-open`, `data-side` and `data-align` wherever they
come from.

## Browser notes

A few components lean on newer CSS. Everything else works everywhere Dioxus
does.

- Most of [`surface`] reads the content beneath it with `backdrop-filter`.
  Where that is unsupported each effect falls back to a plain translucent
  layer — dimmer, but never blank.
- [`scroll::ProgressIndicator`] uses `animation-timeline: scroll()`. Where that
  is unsupported the bar stays at zero width rather than showing a misleading
  full bar — treat it as progressive enhancement.
- [`cursor::MouseFollow`] and [`cursor::CursorTrail`] need the pointer position
  anywhere on the page, so they install one shared `pointermove` listener that
  publishes it as CSS custom properties. They are inert outside a browser.
- Several loaders use `color-mix()` for their track colour and fall back to a
  neutral grey without it.

## Design notes

- **Easing, not springs.** A spring has to be simulated per frame, and CSS has
  no equivalent. Anything springy here is an overshooting cubic-bezier, which
  reads the same at these durations and costs nothing to run.
- **Colours are colours.** Every colour prop takes a CSS colour, not a utility
  class, so nothing here needs Tailwind or any other framework.
- **Deterministic by default.** Nothing randomises itself per render:
  `AppleEqualizer` uses a fixed uneven set of delays, `WavePhysicsLoader` runs
  its physics once at startup and emits the result as keyframes, and `Blaze`
  scatters its sparks with fixed strides. A rebuild of the same page animates
  the same way.
- **Icons are yours to supply.** [`buttons::AnimatedButton`] takes `icon` and
  `alt_icon` as markup, so the crate depends on no icon set.

## Licence

MIT or Apache-2.0, at your option.

The loading, entrance, text, hover, cursor, scroll, button and card components
began as a port of [Amicro](https://github.com/Subhan-code/Amicro--Micro-transitions-),
which is MIT-licensed.
