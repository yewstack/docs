# 第一个简单的App

首先创建一个二进制项目:

```bash
cargo new --bin yew-app && cd yew-app
```

添加 `yew` 到你的依赖库中 \([这里](https://docs.rs/yew) 可以查看最新版本的 Yew\)

{% code title="Cargo.toml" %}
```text
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[dependencies]
yew = "0.11.0"
```
{% endcode %}

将这份代码复制到你的 `src/main.rs` 文件中:

{% code title="src/main.rs" %}
```rust
use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <button onclick=&self.onclick>{ button_text }</button>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
```
{% endcode %}

这份代码将构建你的称为`App` 的 `Component` 组件, 他会显示一个按钮, 当你点击它时, `App` 将会更新自己的状态. `yew::start_app::<App>()` 会启动你的应用并加载到 `<body>` 标签中. If you would like to start your application with any dynamic properties, you can instead use `yew::start_app_with_props(..)`.

## 运行!

启动并运行你的应用的最快方式就是使用 [`cargo-web`](https://github.com/koute/cargo-web) . 如果你还没有的话, 请用 `cargo install cargo-web` 命令安装.

运行一个用于开发的服务器来执行你的程序:

```bash
cargo web start
```

`cargo-web` 将会自动为你添加 `wasm32-unknown-unknown` 作为目标代码, 然后构建你的应用, 你的应用将默认在 [http://\[::1\]:8000](http://[::1]:8000) 被访问. 可以通过 `cargo web start --help` 命令来获取更多选项和帮助.

