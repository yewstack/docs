# Using wasm-pack

This tool was created by the Rust / Wasm Working Group and is the most actively developed tool for building WebAssembly applications. It supports packaging code into `npm` modules and has an accompanying [Webpack plugin](https://github.com/wasm-tool/wasm-pack-plugin) for easy integration with an existing JavaScript application. Find more information [here](https://rustwasm.github.io/docs/wasm-pack/introduction.html).

{% hint style="info" %}
Note that the crate-type in your `Cargo.toml` will need to be `cdylib` when using `wasm-pack`
{% endhint %}

### Install wasm-pack

```bash
cargo install wasm-pack
```

### Set up your project

{% hint style="info" %}
If you use the [yew-wasm-pack-minimal](https://github.com/yewstack/yew-wasm-pack-minimal) template, these setup steps are already taken care of.
{% endhint %}

* Add a minimal `index.html` file to your project root:
```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <title>Yew</title>
    </head>
    <body>
        <script src="pkg/bundle.js"></script>
    </body>
</html>
```

* Add a `main.js` file with these contents to your project root:
```javascript
import init, { run_app } from './pkg/myapp.js';
async function main() {
   await init('/pkg/myapp_bg.wasm');
   run_app();
}
main()
```

* Add `wasm-bindgen` as a dependency in `Cargo.toml`:
```toml
[dependencies]
wasm-bindgen = "^0.2"
```

* Add a `run_app` function to `lib.rs`:
```rust
use wasm_bindgen::prelude;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Model>();

    Ok(())
}
```

### Build

This command will produce a bundle in the `./pkg` directory with your app's compiled WebAssembly along with a JavaScript wrapper which can be used to start your application.

```bash
wasm-pack build --target web
```

### Bundle

You need the `wasm` plugin for `rollup`:

```bash
npm install @rollup/plugin-wasm --save-dev
```

Now you can bundle your application:

```bash
rollup ./main.js --format iife --plugin wasm --file ./pkg/bundle.js
```

For more information on Rollup visit this [guide](https://rollupjs.org/guide/en/#quick-start).

### Serve

Feel free to use your preferred server. Here we use a simple python server to serve to [http://\[::1\]:8000](http://[::1]:8000).

```bash
python -m http.server 8000
```

### Supported Targets

* `wasm32-unknown-unknown`

