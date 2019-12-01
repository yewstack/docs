---
description: The procedural macro for generating HTML
---

# html! 宏

## 在RUST中使用HTML

`html!` 宏 允许你在Rust中使用HTML代码，并且修改并且拓展原有的HTML语法，这和在React中广泛使用的JSX语法有些相似。

{% hint style="info" %}
`注：Html<COMP> 是 VNode<COMP> 的别名`
{% endhint %}

### 空标签

`html!` 宏总是要求单个HTML标签作为根节点。接下来的一些例子，都会让编译器抛出错误： `error: only one root html element allowed`

```rust
// 错误
html! {
    <div></div>
    <p></p>
}
```

为了解决这个问题，Yew允许使用空标签，使用  `<></>` 来定义。可以在 `html!` 宏中 使用空标签来作为一系列的HTML标签的根标签。

```rust
// 正确
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

### HTML标签

HTML标签的写法与HTML标准或是SVG标准大致符合，有少许的不同。例如，在宏中，一个HTML标签要么是自闭合的标签

```rust
html! {
    // 错误 (没有自闭合标签)
    <input id="my_input">
}

html! {
    // VALID
    <input id="my_input" />
}
```

或者是一个开始标签，但是一定要有与之相对应的结束标签。

```rust
html! {
    // INVALID (MISSING CLOSE TAG)
    <div id="my_div">
}

html! {
    // VALID
    <div id="my_div"></div>
}
```

### 表达式

你可以在HTML中使用 `{}` 块来插入Rust表达式，只要这些表达式最终可以被解析成`Html<_>`

```rust
html! {
  <div>
    {
      if show_link {
        html! {
          <a href="https://example.com">{"Link"}</a>
        }
      } else {
        html! {}
      }
    }
  </div>
}
```

通常我们会把这些表达式写进函数或者闭包中来增加可读性和可维护性。

```rust
let show_link = true;
let maybe_display_link = move || -> Html<SomeComponent> {
  if show_link {
    html! {
      <a href="https://example.com">{"Link"}</a>
    }
  } else {
    html! {}
  }
};

html! {
     <div>{maybe_display_link()}</div>
}
```

### 常量

如果一个表达式的类型本身实现了 `Display` （一个标准库中的Trait），他们将会被转化成字符串并且作为一个Text节点插入DOM树中。

所有的需要显示的文本必须被 `{}` 块包含，因为这些文本应该被认为是一个Rust的表达式来处理。这一点上，Yew中使用HTML的方式和正常HTML语法有巨大的区别。

```rust
let text = "lorem ipsum";
html!{
    <>
        <div>{text}</div>
        <div>{"dolor sit"}</div>
    </>
}
```

### 回调

声明在`html!` 宏中的闭包自动的被认为是回调`Callbacks`。回调`Callbacks`，会将信息返回给所属的组件，他们通常用来接收来自子组件的信息，或者是处理像是 `input`和 `button`的HTML标签的事件。

```rust
pub enum Msg {
    ButtonClicked
}

html!{
    <button onclick=|_| Msg::ButtonClicked>{ "Click Me!" }</button>
}
```

If the message you want your callback to return _wraps_ the argument in the closure in a tuple-variant, you can use the function tuple syntax instead, but only for `Component`s, and not for plain elements \([Issue](https://github.com/yewstack/yew/issues/733)\)

```rust
pub enum Msg {
    ButtonClicked(ClickEvent)
}

html! {
    <ButtonComponent callback=Msg::ButtonClicked />
}
```

This extends to the case if the argument is the same as the message you want to capture:

```rust
html! {
    <ButtonComponent callback=From::from></ButtonComponent>
}

// or
html! {
    <ButtonComponent callback=std::convert::identity />
}
```

### 组件

阅读下面的章节，来学习如何更好的在 `html!` 中使用组件。

