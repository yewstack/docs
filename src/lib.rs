use doc_comment::{doc_comment, doctest};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_sample_app() {
    doc_comment!(include_str!(concat!(
        env!("OUT_DIR"),
        "/getting-started/build-a-sample-app.md"
    )));
}

#[wasm_bindgen_test]
fn test_optimizations() {
    doctest!("advanced-topics/optimizations.md");
}

#[wasm_bindgen_test]
fn test_properties() {
    doctest!("concepts/components/properties.md");
}

#[wasm_bindgen_test]
fn test_component_readme() {
    doctest!("concepts/components/README.md");
}

#[wasm_bindgen_test]
fn test_component_internalstate() {
    doctest!("concepts/components/internalstate.md");
}