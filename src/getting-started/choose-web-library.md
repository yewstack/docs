# 选择 web-sys 还是 stdweb

### 简介

Yew 应用程序可以通过 [`web-sys`](https://docs.rs/web-sys) 或者 [`stdweb`](https://docs.rs/stdweb) 来构建。这两个 crates 提供了 Rust 和 Web API 之间的绑定。当把 `yew` 添加到你的 cargo 依赖时，你需要选择它们其中之一：

{% code title="Cargo.toml" %}
```rust
# 选择 `web-sys`
yew = { version = "0.13", features = ["web_sys"] }

# 选择 `stdweb`
yew = { version = "0.13", features = ["std_web"] }
```
{% endcode %}

我们建议选择 `web-sys`，因为它是由 [Rust / Wasm 工作组](https://rustwasm.github.io/) 提供支持。

### 示例用法

```rust
// web-sys
let window: web_sys::Window = web_sys::window().expect("window not available");
window.alert_with_message("hello from wasm!").expect("alert failed");

// stdweb
let window: stdweb::web::Window = stdweb::web::window();
window.alert("hello from wasm!");

// stdweb 搭配 js! 宏
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Window;

let window_val: stdweb::Value = js!{ return window; }; // <- 里面使用 JS 语法
let window = Window::try_from(window_val).expect("conversion to window failed");
window.alert("hello from wasm!");
```

两个 crate 的 API 略有不用，但他们的目标大致相同，功能相似。

### 选择其中之一

当为你的应用程序选择使用 `web-sys` 还是 `stdweb` 时，有几个不同的角度需要考虑。注意，可以在一个应用程序中同时使用两者，但是为了最小化编译的 `.wasm` 二进制体积，最好选择其中之一。


<table>
  <thead>
    <tr>
      <th style="text-align:left"></th>
      <th style="text-align:left"><code>web-sys</code>
      </th>
      <th style="text-align:left"><code>stdweb</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align:left">项目状态</td>
      <td style="text-align:left">由<a href="https://rustwasm.github.io/">Rust / Wasm 工作组积极维护</a>
      </td>
      <td style="text-align:left">超过四个月没有 Github 活动</td>
    </tr>
    <tr>
      <td style="text-align:left">Web API 覆盖率</td>
      <td style="text-align:left">Rust API 是从 Web IDL 规范自动生成，因此理论上有 100% 的覆盖率。</td>
      <td style="text-align:left">浏览器 API 是根据需求由社区添加</td>
    </tr>
    <tr>
      <td style="text-align:left">Rust API 设计</td>
      <td style="text-align:left">采取保守的方法，为大多数 API 调用返回 <code>Result</code></td>
      <td style="text-align:left">通常拒绝返回 <code>Result</code> 而更倾向于使用 panic。例如，在 worker 中调用 <code>stdweb::web::window()</code> 将 panic。</td>
    </tr>
    <tr>
      <td style="text-align:left">支持的构建工具</td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>cargo-web</code>
          </li>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">支持生成的目标代码</td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
          <li><code>wasm32-unknown-emscripten</code>
          </li>
          <li><code>asmjs-unknown-emscripten</code>
          </li>
        </ul>
      </td>
    </tr>
  </tbody>
</table>

有关更多挑选构建工具的信息，请参阅以下页面：
