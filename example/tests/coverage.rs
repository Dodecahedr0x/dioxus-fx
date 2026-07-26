//! Every component in the library must have a showcase story.
//!
//! The gallery is generated from `#[story]` annotations, so a component with no
//! story silently disappears from it. This test reads both sides as source and
//! fails with the names that are missing, rather than letting the gap ship.

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
/// canvas. `StoryStage` mounts this one around every preview instead.
const NOT_SHOWCASED: [&str; 1] = ["MicroTransitionsStyle"];

/// Every `#[component] pub fn Name` exported by the library.
fn library_components() -> Vec<String> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../dioxus-micro-transitions/src");
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

/// Every component named by a `#[story(title = "...")]` in this crate.
fn showcased_components() -> Vec<String> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut names = Vec::new();
    for file in rust_files(&src) {
        let source = fs::read_to_string(&file).expect("read story source");
        for block in source.split("#[story(title = \"").skip(1) {
            let title = block.split('"').next().expect("story title is terminated");
            // Titles read `Category/Component` or `Category/Component/Variant`;
            // the component is always the second segment.
            let component = title
                .split('/')
                .nth(1)
                .expect("story title names a component");
            names.push(component.to_owned());
        }
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
