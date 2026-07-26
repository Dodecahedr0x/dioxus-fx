//! Every component in the library must have a showcase story, and every story
//! written must reach the gallery.
//!
//! Those are two different facts. A component with no `#[story]` silently
//! disappears from the gallery, which the source scans below catch. Separately,
//! since dioxus-showcase 0.1.0 a story reaches the shell by registering itself
//! at link time rather than through generated glue, so a story can be written,
//! compile, and still never appear — with no error anywhere. The registry
//! assertions at the bottom catch that.

// LOAD-BEARING, not an unused import. Integration tests link this crate as an
// rlib, and without a reference to it the linker never selects its archive
// member, dropping every `inventory` registration inside it — the same
// mechanism the generated showcase app pins `lto` to defeat on wasm32. Remove
// this line and the registry reads as empty rather than failing to build.
use gallery as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Collects `.rs` files under a directory, recursively.
fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).expect("read source directory") {
        let path = entry.expect("read directory entry").path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    files
}

/// Components that render no visible markup, so a story would show an empty
/// canvas. Both mount stylesheets: `StoryStage` renders the first around every
/// preview, and the `Primitives/…` stories render the second themselves.
const NOT_SHOWCASED: [&str; 2] = ["FxStyle", "PrimitivesStyle"];

/// Every `#[component] pub fn Name` exported by the library.
fn library_components() -> Vec<String> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../dioxus-fx/src");
    let mut names = Vec::new();
    for file in rust_files(&src) {
        let source = fs::read_to_string(&file).expect("read library source");
        for block in source.split("#[component]\npub fn ").skip(1) {
            let name: String = block.chars().take_while(|c| c.is_alphanumeric()).collect();
            if !NOT_SHOWCASED.contains(&name.as_str()) {
                names.push(name);
            }
        }
    }
    names.sort();
    names
}

/// Every `#[story(title = "...")]` title written in this crate's source.
fn fs_story_titles() -> Vec<String> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut titles = Vec::new();
    for file in rust_files(&src) {
        let source = fs::read_to_string(&file).expect("read story source");
        for block in source.split("#[story(title = \"").skip(1) {
            let title = block.split('"').next().expect("story title is terminated");
            titles.push(title.to_owned());
        }
    }
    titles
}

/// Every component named by a `#[story(title = "...")]` in this crate.
fn showcased_components() -> Vec<String> {
    let mut names = Vec::new();
    for title in fs_story_titles() {
        // The `Primitives` category demonstrates the state-attribute stylesheet
        // on components from `dioxus-primitives`, so its titles name those
        // rather than anything this workspace defines.
        if title.starts_with("Primitives/") {
            continue;
        }
        // Every other title reads `Category/Component` or
        // `Category/Component/Variant`; the component is the second segment.
        let component = title
            .split('/')
            .nth(1)
            .expect("story title names a component");
        names.push(component.to_owned());
    }
    names.sort();
    names.dedup();
    names
}

#[test]
fn every_library_component_has_a_story() {
    let components = library_components();
    assert!(
        components.len() > 150,
        "found only {} components; the scan is broken",
        components.len()
    );

    let showcased = showcased_components();
    let missing: Vec<&String> = components
        .iter()
        .filter(|name| !showcased.contains(name))
        .collect();

    assert!(
        missing.is_empty(),
        "components with no showcase story: {missing:?}"
    );
}

#[test]
fn every_story_names_a_real_component() {
    let components = library_components();
    let stale: Vec<String> = showcased_components()
        .into_iter()
        .filter(|name| !components.contains(name))
        .collect();

    assert!(
        stale.is_empty(),
        "stories for components that no longer exist: {stale:?}"
    );
}

/// Every story title the macros actually registered at link time.
///
/// The source scan above proves a `#[story]` was *written*; this proves it was
/// *registered*. Since 0.1.0 those are separate facts — stories reach the shell
/// through `inventory` rather than through generated glue, and a registration
/// that fails to link produces an empty gallery with no error anywhere, which is
/// indistinguishable from having annotated nothing.
fn registered_titles() -> Vec<String> {
    let mut titles: Vec<String> = dioxus_showcase::registered_stories()
        .stories
        .into_iter()
        .map(|story| story.definition.title)
        .collect();
    titles.sort();
    titles.dedup();
    titles
}

#[test]
fn the_story_registry_is_not_empty() {
    let registered = registered_titles();
    assert!(
        !registered.is_empty(),
        "no stories registered: the gallery would render empty with no error"
    );
}

#[test]
fn every_written_story_reaches_the_registry() {
    // `showcased_components()` reads titles out of the source; this reads them
    // out of the linked binary. A title in one and not the other means a story
    // was written but never registered, or registered under another name.
    let mut written: Vec<String> = fs_story_titles();
    written.sort();
    written.dedup();

    let registered = registered_titles();
    let missing: Vec<&String> = written.iter().filter(|t| !registered.contains(t)).collect();
    let extra: Vec<&String> = registered.iter().filter(|t| !written.contains(t)).collect();

    assert!(
        missing.is_empty(),
        "written but never registered: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "registered but not found in source: {extra:?}"
    );
}

#[test]
fn no_two_stories_claim_the_same_id() {
    // Colliding ids stopped panicking in 0.1.0: the shell renders a banner and
    // both stories stay navigable, so a collision is now easy to ship unnoticed.
    let duplicates = dioxus_showcase::registered_stories().duplicate_ids;
    assert!(duplicates.is_empty(), "duplicate story ids: {duplicates:?}");
}

/// The parameter list of each `#[story]` function in `source`, unsplit.
///
/// Scans to the matching `)` rather than the first one: a default such as
/// `#[default = "rgba(255,255,255,.08)"]` puts parentheses inside the list.
fn story_parameter_lists(source: &str) -> Vec<(String, String)> {
    let mut lists = Vec::new();
    for block in source.split("#[story(").skip(1) {
        let Some(fn_at) = block.find("fn ") else {
            continue;
        };
        let rest = &block[fn_at + 3..];
        let Some(open) = rest.find('(') else { continue };
        let name = rest[..open].trim().to_owned();

        let (mut depth, mut in_str, mut end) = (0usize, false, None);
        for (i, c) in rest[open..].char_indices() {
            match c {
                '"' => in_str = !in_str,
                '(' if !in_str => depth += 1,
                ')' if !in_str => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(open + i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { continue };
        lists.push((name, rest[open + 1..end].to_owned()));
    }
    lists
}

/// Splits a parameter list on the commas that separate parameters, ignoring
/// those inside a default expression or a string.
fn split_parameters(list: &str) -> Vec<String> {
    let (mut out, mut depth, mut in_str, mut start) = (Vec::new(), 0usize, false, 0usize);
    for (i, c) in list.char_indices() {
        match c {
            '"' => in_str = !in_str,
            '(' | '[' if !in_str => depth += 1,
            ')' | ']' if !in_str => depth = depth.saturating_sub(1),
            ',' if !in_str && depth == 0 => {
                out.push(list[start..i].trim().to_owned());
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(list[start..].trim().to_owned());
    out.into_iter().filter(|p| !p.is_empty()).collect()
}

#[test]
fn every_story_parameter_declares_what_its_control_opens_on() {
    // A parameter with no `#[default = …]` opens its control on `StoryArg`'s
    // placeholder seed — `0`, `false`, `"Lorem Ipsum"` — which is a value the
    // preview is not rendering. The control then disagrees with the component
    // beside it, silently, for as long as nobody looks closely.
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut bare = Vec::new();

    for file in rust_files(&src) {
        let source = fs::read_to_string(&file).expect("read story source");
        for (story, list) in story_parameter_lists(&source) {
            for param in split_parameters(&list) {
                if !param.starts_with("#[default") {
                    bare.push(format!("{story}({param})"));
                }
            }
        }
    }

    assert!(
        bare.is_empty(),
        "story parameters with no #[default], so their controls open on a \
         placeholder rather than on the value the preview shows: {bare:#?}"
    );
}
