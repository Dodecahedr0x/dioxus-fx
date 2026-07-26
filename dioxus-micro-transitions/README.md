# dioxus-micro-transitions

Reusable micro-interactions and transition components for [Dioxus](https://dioxuslabs.com),
ported from [Amicro](https://github.com/Subhan-code/Amicro--Micro-transitions-).

All 155 components from the Amicro registry, plus its button and card
interaction sets:

| Module | What's in it |
| --- | --- |
| [`loading`] | 134 spinners, progress rings, skeletons and activity indicators |
| [`entrance`] | 7 mount-time reveals — fade, slide, scale, zoom |
| [`text`] | 4 staggered text reveals — by character, word or line |
| [`hover`] | 4 pointer-reactive pieces — glow, magnetic, tilt, card grid |
| [`cursor`] | 3 cursor followers — spotlight, ring, trail |
| [`scroll`] | 3 scroll effects — progress bar, reveal, sticky reader |
| [`buttons`] | 12 button interactions behind one component, plus a focus-blur link row |
| [`cards`] | 9 hover-fanned card layouts and 3 carousels |
| [`primitives`] | Enter/exit animations for headless component libraries |

## Design

**No animation runtime.** Upstream is built on Framer Motion. This crate has no
JavaScript animation library, no CSS framework, and exactly one dependency —
`dioxus` itself. Every animation is CSS keyframes or transitions, so it runs on
the compositor rather than the main thread.

**Nothing to set up.** Each component injects the small stylesheet it needs into
`<head>` on first use, deduplicated by key. Use a component and it works. If you
would rather ship the CSS yourself, [`stylesheet`] hands you the whole thing as a
string and [`MicroTransitionsStyle`] mounts it in one place.

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
dioxus-micro-transitions = "0.1"
```

## Use

```rust, no_run
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

Colours default to `currentColor`, so a loader picks up the text colour around
it:

```rust, no_run
# use dioxus::prelude::*;
# use dioxus_micro_transitions::prelude::*;
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
dioxus-micro-transitions = { version = "0.1", default-features = false, features = ["loading"] }
```

`loading`, `entrance`, `text`, `hover`, `cursor`, `scroll`, `buttons`, `cards`,
`primitives`.

## With dioxus-primitives

[`dioxus-primitives`](https://github.com/DioxusLabs/dioxus-components) — the
crate behind <https://dioxuslabs.com/components> — ships unstyled components
that describe their state with data attributes. The [`primitives`] module is a
stylesheet keyed on those attributes, so animating one is adding a class:

```rust, no_run
# use dioxus::prelude::*;
use dioxus_micro_transitions::primitives::*;

fn App() -> Element {
    rsx! {
        PrimitivesStyle {}
        // DialogRoot   { class: "dx-dialog-backdrop {AMT_FADE}", .. }
        // DialogContent{ class: "dx-dialog {AMT_ZOOM}", .. }
        // PopoverContent { class: "{AMT_SLIDE}", .. }   // follows data-side
        // AccordionContent { class: "{AMT_COLLAPSE}", .. }
    }
}
```

Enter *and* exit: the primitives hold closing content in the DOM until its
animation finishes. Nothing here depends on `dioxus-primitives` — the rules
match `data-state`, `data-open`, `data-side` and `data-align` wherever they
come from.

## Browser notes

Three components lean on newer CSS. Everything else works everywhere Dioxus
does.

- [`scroll::ProgressIndicator`] uses `animation-timeline: scroll()`. Where that
  is unsupported the bar stays at zero width rather than showing a misleading
  full bar — treat it as progressive enhancement.
- [`cursor::MouseFollow`] and [`cursor::CursorTrail`] need the pointer position
  anywhere on the page, so they install one shared `pointermove` listener that
  publishes it as CSS custom properties. They are inert outside a browser.
- Several loaders use `color-mix()` for their track colour and fall back to a
  neutral grey without it.

## Differences from upstream

- Spring physics become the closest cubic-bezier. Springs are simulated
  per-frame in JavaScript; CSS has no equivalent, and an overshooting easing
  curve reads the same at these durations.
- Tailwind class props become real CSS colours, so the crate does not require
  Tailwind. `bg-blue-500` becomes `#3b82f6`, and `bg-zinc-800 dark:bg-white`
  becomes `currentColor`.
- `AppleEqualizer` used `Math.random()` for its bar delays, which made every
  render differ. This port uses a fixed uneven set.
- `WavePhysicsLoader` computed 201 animation frames in JavaScript on every
  render. This port runs the same physics once at startup and emits the result
  as CSS keyframes.
- `CardTimeMachine` uses `border-radius` in place of upstream's SVG squircle
  filter.
- Icons are yours to supply. [`buttons::AnimatedButton`] takes `icon` and
  `alt_icon` as markup rather than depending on an icon set.

## Licence

MIT or Apache-2.0, at your option. Upstream Amicro is MIT-licensed.
