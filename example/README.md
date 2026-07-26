# gallery

Showcase stories for every component in `dioxus-micro-transitions`.

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
| `src/loading.rs` | 134 loader stories |
| `src/entrance.rs`, `src/text.rs`, … | One module per library module |
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

Title segments drive the navigation tree: `Category/Component`, plus a third
segment when one component covers several variants
(`Buttons/AnimatedButton/Sparkle`). `tests/coverage.rs` reads the second segment
back, so it has to be the component's name.
