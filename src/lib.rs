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
fn test_components_properties() {
    doctest!("concepts/components/properties.md");
}
#[wasm_bindgen_test]
fn test_components_emitevents() {
    doctest!("concepts/components/emitevents.md");
}
#[wasm_bindgen_test]
fn test_components_internalstate() {
    doctest!("concepts/components/internalstate.md");
}
#[wasm_bindgen_test]
fn test_components_trapevents() {
    doctest!("concepts/components/trapevents.md");
}
#[wasm_bindgen_test]
fn test_components_readme() {
    doctest!("concepts/components/README.md");
}
