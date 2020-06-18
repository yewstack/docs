---
description: The procedural macro for generating HTML and SVG
---

# Using [`html!`][1]

The [`html!`][1] macro allows you to write HTML and SVG code declaratively. It is similar to JSX \(a Javascript extension which allows you to write HTML code inside of Javascript\).

**Important notes**


- The [`html!`][1] macro only accepts one root HTML node (you can overcome this by [using fragments or iterators](lists.md))
- An empty `html! {}` invocation is valid and will not render anything.
- Literals inside of tags must always be quoted and wrapped with braces (this is different to attribute values - see below)
  * `html! { "Hello, World" }`
  * `html! { <div>{ "Hell, World" }</div> }`
  * `html! { <div>{ String::from("foo") + "bar" }</div>`
- Quoted attribute values are taken literally. The value is set at compile-time and does not change at run-time.
  * `html! { <div> id="bar"</div> }`
- Unquoted attribute values are interpreted as expressions and therefore have to be valid Rust expressions.
  * `let foo = "bar"; html! { <div id=foo></div> }`
  * `html! { <div id=String::from("foo") + "bar"></div> }`
- HTML tags names need to start with a lowercase letter. Besides that every valid HTML tag name is allowed. If the tag name starts with an uppercase letter it is interpreted as component name and attributes are passed as component properties.

{% hint style="info" %}
The [`html!`][1] macro can reach easily the default recursion limit of the compiler. It is advised to bump its value if you encouter compilation errors. Use an attribute like `#![recursion_limit="1024"]` to bypass the problem. See the [official documentation][2] and [this Stack Overflow question][3] for details.
{% endhint %}

{% page-ref page="lists.md" %}

{% page-ref page="elements.md" %}

{% page-ref page="literals-and-expressions.md" %}

{% page-ref page="components.md" %}

[1]: https://docs.rs/yew/latest/yew/macro.html.html
[2]: https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute
[3]: https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it
