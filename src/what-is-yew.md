---
description: 框架的高级概述。
---

# Yew 是什么？

## Yew 是什么？

Yem 是一个前端 Web 框架，类似于 React 和 Elm，允许你使用复杂的逻辑来构建运行在浏览器上的网站。使用 Yew 的应用程序是由 Rust 编写的，可编译为 Web Assembly \(**WASM**\)，或纯 JavaScript，以便在浏览器上运行。

### 价值主张

由于 Rust 几乎不存在运行时（导致较小的文件大小），并且其各种语言特性可以安全地将 WASM 用作目标，同时又能实现最佳性能，Rust 是可编译为 WASM 的最佳语言。Yew 是一个 Rust/WASM 框架，使用过 React 或者 Elm 的人应该熟悉它的架构，它使你能构建快速，小巧和正确的前端 Web 应用。

### 是由什么构成了 Yew ？

Yew 包含几个不同的部分来创建一个可工作的应用。

- `html!` macro - 一个宏程序，它创建一棵树，该树代表将在浏览器中显示的 HTML。
- `Component` trait - 指定 Rust 中的数据结构是如何在浏览器中显示以及与之交互。
- `Properties` trait - 允许组件（component） 传递状态给子组件（chind component）。
- `Callback` event system - 允许子组件（chind component），actor 或者 HTML 元素向组件（component）发送消息。
- `Agent` trait - 指定能够协调全局状态或者能够在 web worker 中运行独立任务的 actor。
- `Services` - 浏览器中存在的 API 的 Rust 胶水代码。包括：fetch requests, timers, console access 等等。

#### 依赖

Yew 建立在 `StdWeb` 之上，这个库提供了 Rust 和浏览器之间的绑定。一些功能依赖于另一个叫做 `web-sys` 的库，该库是从 web 浏览器规范文档中自动生成的，并使用了 `wasm_bindgen`。

#### 构建环境

如果你的应用程序被设计为仅使用基于 `StdWeb` 的功能，则可以使用 `cargo-web` 构建工具来构建，测试并运行你的应用程序。如果你想使用高级功能，或者只是青睐生态，则可以使用各种现有的 JS 打包器及其基于 `wasm_bindgen` 的插件来构建你的应用程序。其中包括使用 `wasm-pack` 进行构建，并使用 `rollup` 来打包，或者使用 `Webpack` 或 `parcel` 来管理你的开发和部署任务。

`cargo-web` 支持通过 `Emscripten` 编译为 JS 或者使用 `rustc`编译为 WASM，而基于 `wasm_bindgen` 的方法仅支持编译为 WASM。
