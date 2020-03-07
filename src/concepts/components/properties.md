---
description: 父组件到子组件的通信
---

# 属性（Properties）

如“组件（Components）”页面所述，Properties 用于父级到子组件的通信。

## 派生宏

不要尝试自己去实现 `Properties`，而是通过使用 `#[derive(Properties)]` 来派生它。

### 必需属性

实现了 `Properties` 的结构体中的字段必须是 `Default` 或者应用了 `#[props(required)]` 属性。这个属性向框架发出信号，要求在 `html!` 宏中创建组件时必须提供该字段，否则将收到编译错误。对于非必需字段，通常是将它们包装在 `Option` 中，当不提供该字段时，默认为 `None`。

### PartialEq

如果可以的话，在你的 props 上派生 `PartialEq` 通常是很有意义的。这使用了一个**优化和最佳实践**部分解释了的技巧，可以更轻松地避免重新渲染。

## Properties 的内存/速度开销

记住组件的 `view` 函数签名：

```rust
fn view(&self) -> Html
```

你对组件的状态取了一个引用，并用来创建 `Html`。但是 properties 是有所有权的值（owned values）。这意味着为了创造它们并且将它们传递给子组件，我们需要获取 `view` 函数里提供的引用的所有权。这是在将引用传递给组件时隐式克隆引用完成的，以获得构成其 props 的有所有权的值。

这意味着每个组件都有从其父级传递来的状态的独特副本，而且，每当你重新渲染一个组件时，该重新渲染组件的所有子组件的 props 都将被克隆。

这意味着如果你将 _大量_ 数据作为 props（大小为 10 KB 的字符串）向下传递，则可能需要考虑将子组件转换为在父级运行返回 `Html` 的函数，因为这样就不会被强制克隆你的数据。

另外，如果你不需要修改作为 props 传递的大数据，而只需要显示它，则可以将其包装在 `Rc` 中，以便仅克隆一个引用计数的指针，而不是数据本身。

## 示例

```rust
pub struct LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

impl Default for LinkColor {
    fn default() -> Self {
        // 除非另有说明，否则链接的颜色将为蓝色
        LinkColor::Blue
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    /// 链接必须有一个目标地址
    #[props(required)]
    href: String,
    /// 如果链接文本很大，这将使得复制字符串开销更小
    /// 除非有性能问题，否则通常不建议这么做
    #[props(required)]
    text: Rc<String>,
    /// 链接的颜色
    color: LinkColor,
    /// 如果为 None，则 view 函数将不指定大小
    size: Option<u32>
}
```

