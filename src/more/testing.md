---
description: Testing your app
---

# Testing

&lt;TODO&gt;

## Rust WebDriving

For programmatically driving UI integration testing using Rust, [fantoccini](https://crates.io/crates/fantoccini) is a recommended choice. It allows you to test your website by finding specific elements using CSS selectors, and then performing actions on them like inputting text, clicking buttons, or waiting for specific amounts of time during which the client can execute code \(e.g. wait for a fetch request to finish and cause a UI change\).

## wasm_bingen_test
The Rust WASM working group maintains a crate called [`wasm_bindgen_test`](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html) which allows you to run tests in a browser in similar fashion to how the built-in `#[test]` procedural macro works. More information is given in the [Rust WASM working group's documentation](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html) for this module.
