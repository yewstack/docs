# 构建初始环境

## 开发工具

事实上你并不需要额外的构建工具来发布Yew应用，但是我们推荐你使用一个。他们可以让部署和打包少去很多令人头疼的环节，比如为你生成所有必要的与 `.wasm` 对应的 JavaScript 文件，来让应用能顺利的跑在浏览器中。

你可以通过浏览 [Starter Templates](starter-templates.md) 来快速上手并开始一个应用，或者是继续阅读来查看更多有关开发工具的信息。

## `wasm-pack`

这个工具由 Rust / Wasm 小组开发维护，并且是现在最为活跃的WebAssembly 应用开发工具。 它支持将代码打包成 npm 模块，并且随附了 [Webpack plugin](https://github.com/wasm-tool/wasm-pack-plugin) 插件 可以轻松的与已有的JavaScript应用结合。可以点击此处了解更多 [here](https://rustwasm.github.io/docs/wasm-pack/introduction.html).

{% hint style="info" %}
注：如果使用 `wasm-pack`作为开发工具`，Cargo.toml`中的 `crate-type`需要为 `cdylib`
{% endhint %}

### **安装**

```bash
cargo install wasm-pack
```

### 构建

这条命令将在工程根目录下 的`./pkg` 目录中生成打包后的应用，其中包含应用的WebAssembly文件以及用来启动应用的JavaScript包装器。

```bash
wasm-pack build
```

### 打包

关于 Rollup 的更多信息，请查看： [guide](https://rollupjs.org/guide/en/#quick-start)

```bash
rollup ./main.js --format iife --file ./pkg/bundle.js
```

### 部署

选取你喜爱的服务器，这里我们使用一个简单的 Python 服务器来将项目部署到： [http://\[::1\]:8000](http://[::1]:8000).

```bash
python -m SimpleHTTPServer 8080
```

### 支持生成的目标代码

* `wasm32-unknown-unknown`

## `cargo-web`

Cargo web 是一个用来构建web应用的 Cargo 子命令，它可以让构建和部署web应用变得非常的简单。他同样也是唯一一个支持生成 Emscripten 目标代码的工具链，点击这里了解更多：[here](https://github.com/koute/cargo-web)

### **安装**

```bash
cargo install cargo-web
```

### 构建

```bash
cargo web build
```

### 运行

```bash
cargo web start
```

### 支持生成的目标代码

* `wasm32-unknown-unknown`
* `wasm32-unknown-emscripten`
* `asmjs-unknown-emscripten`

{% hint style="info" %}
对于 `*-emscripten` 的目标代码， `cargo-web` 将会自动安装 Emscripten SDK 并且为你生成目标代码。
{% endhint %}

