---
description: Set yourself up for success
---

# 安装

## Rust

首先 ，你需要安装 Rust 。

如何安装 Rust 和 `cargo` 工具链, 请参考 [**official instructions**](https://www.rust-lang.org/tools/install).

## **开发工具**

[**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/) 是一个由Rust-Wasm 小组开发，并且作为开发应用的**推荐工具**，参考官方的 [**install instructions**](https://rustwasm.github.io/wasm-pack/installer/) 来进行安装。

[**`cargo-web`**](https://github.com/koute/cargo-web) 是 `wasm-pack`  出现之前的首选开发工具，它现在仍然是让你的应用运行起来的 **最快** 的方式。并且很多示例项目还没有迁移到 `wasm-pack` 上，这点上看，它也是值得一试的。 

通过键入下列命令来安装：

```bash
cargo install cargo-web
```

