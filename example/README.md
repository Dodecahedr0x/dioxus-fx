# gallery

Showcase stories for every component in `dioxus-fx`.

This crate is the entry crate for
[dioxus-showcase](https://github.com/Dodecahedr0x/dioxus-showcase): it holds no
UI of its own, only `#[story]` annotations that the CLI discovers and turns into
a browsable gallery.

```sh
dioxus-showcase check    # validate the annotations
dioxus-showcase dev      # live gallery at http://127.0.0.1:6111
dioxus-showcase build    # regenerate example/showcase/ without serving
```

`DioxusShowcase.toml` at the repository root points at this crate, so no `init`
step is needed. Everything under `example/showcase/` is generated and ignored by
git apart from its `Cargo.toml`.

## Layout

| Path | What's in it |
| --- | --- |
| `src/lib.rs` | The `num` and `txt` control seeds every story shares |
| `src/stage.rs` | The `#[provider]` wrapper and the demo content stories reuse |
| `assets/photos/` | Placeholder artwork for the carousel stories |
| `src/loading.rs` | 134 loader stories |
| `src/entrance.rs`, `src/text.rs`, … | One module per library module |
| `src/primitives.rs` | The state-attribute add-on, on real `dioxus-primitives` components |
| `tests/coverage.rs` | Fails if a library component has no story |

## Writing a story

A story is a plain function returning `Element`. Its parameters become live
controls in the showcase shell:

```rust
#[story(title = "Loading/IosSpinner", tags = ["loading"])]
pub fn ios_spinner(size: f64, color: String, duration: f64) -> Element {
    rsx! {
        IosSpinner {
            size: num(size, 32.0),
            color: txt(color, "currentColor"),
            duration: num(duration, 1.0),
        }
    }
}
```

Controls are seeded from `dioxus_showcase::StoryArg`, which hands back `0` for
numbers and `"Lorem Ipsum"` for strings — values that would render an invisible
spinner. `num` and `txt` swap an untouched seed for the component's documented
default, so a story opens on the real thing and still responds to the controls.

## Images

`CardCarousel`, `CardCoverFlow` and `CardTimeMachine` are the only components
that take an image URL (`CardItem::src`). They render the five placeholders in
`assets/photos/`, declared as `PHOTOS` in `src/stage.rs`:

```rust
const PHOTOS: [(Asset, &str, &str); 5] = [
    (asset!("/assets/photos/photo-1.svg"), "Sunset", "Today"),
    // ...
];
```

To swap in real artwork, drop the files into `assets/photos/` and update those
paths. `asset!` resolves at compile time, so a different extension means editing
the path — a missing file is a compile error, not a broken image. Anything 3:4
suits the carousel CSS.

## The `Primitives` category

Those stories drive components from
[`dioxus-primitives`](https://github.com/DioxusLabs/dioxus-components) — the
crate behind <https://dioxuslabs.com/components> — with nothing added but a
class from `dioxus_fx::primitives`. Open the dialog or the
popover to see both halves: the enter animation, and the exit one the primitives
wait for before unmounting.

Only a placeholder `0.0.0` of that crate is published, so this crate depends on
the repository at a pinned revision. That dependency is the gallery's alone —
the add-on is a stylesheet keyed on the data attributes those components emit,
so the library itself stays at one dependency.

Their titles read `Primitives/<Effect>` rather than naming a component in this
workspace, which is why `tests/coverage.rs` skips that category.

## Navigation

Title segments drive the navigation tree: `Category/Component`, plus a third
segment when one component covers several variants
(`Buttons/AnimatedButton/Sparkle`). `tests/coverage.rs` reads the second segment
back, so it has to be the component's name.
