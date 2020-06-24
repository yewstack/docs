---
description: Make your app faster.
---

# Optimizations and Best Practices

## neq\_assign

When a component receives props from its parent component, the `change` method is called. The `change` method takes a mutable reference to `self` through which you can make changes to your applications state. It returns a `ShouldRender` (which is currently an alias to `bool`) which should be `true` if your component needs to re-render after the new props have been received and `false` if the component does not need to re-render.

Because re-rendering is computationally expensive, you should avoid it where possible. In general, you should only trigger a re-render when the props have actually changed. An example of how this might be done is given below.

```rust
use yew::ShouldRender;
#[derive(PartialEq)]
struct ExampleProps;

struct Example {
    props: ExampleProps,
};

impl Example {
    fn change(&mut self, props: ExampleProps) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
```
{% endcode %}

But we can go further! This is six lines of boilerplate can be reduced down to one by using a trait and a blanket implementation for anything that implements `PartialEq`. Check out the `yewtil` crate's [`NeqAssign` trait](https://docs.rs/yewtil/*/yewtil/trait.NeqAssign.html).

You aren't limited to using this in the `change` method. It often makes sense to do this in the `update` method as well, although the performance gains are not as obvious there.

## Rc (reference counted variables)

In an effort to avoid cloning large chunks of data to create props when re-rendering, we can use smart pointers to only clone a pointer to the props instead. If you use `Rc<_>`s in your props and child components instead of plain unboxed values, you can delay cloning until you need to modify the data in the child component, where you use `Rc::make_mut` to clone and get a mutable reference to the data you want to alter. By not cloning until mutation, child components can reject props identical to their state-owned props in `Component::change` with almost no performance cost, versus the case where the data itself needs to be copied into the props struct in the parent before it is compared and rejected in the child.

This optimization is most useful for data types that do not implement `Copy`. If you can copy your data easily, then it probably isn't worth putting it behind a smart pointer. For structures that can contain lots of data like `Vec`, `HashMap`, and `String`, this optimization should be worthwhile.

This optimization works best if the values are never updated by the children, and even better, if they are rarely updated by parents. This makes `Rc<_>s` a good choice for wrapping property values in for pure components. 

{% hint style="info" %}
The Rust Book has a [chapter on smart pointers](https://doc.rust-lang.org/stable/book/ch15-00-smart-pointers.html).
{% endhint %}

## View Functions

As you write code which uses the `html!` macro you may find that your code becomes progressively harder to read as you add more code to your component because the amount of indentation increases hugely.

```rust
use yew::prelude::*;
html! {
    <>
        <div class="something">
            <div class="something-child">
                <h1>{"Some child"}</h1>
                <div class="something-child-child">
                    <h4>{"Final thing."}</h4>
                </div>
            </div>
            <a href="#">{"Some link"}</a>
        </div>
    </>
}
```

To make code easier to read, code in `html!` macro calls can be split up into different functions and which can then be called from the `view` function. 

```rust
use yew::prelude::*;
fn something() -> Html {
    html! {
        <div class="something">
            {something_child()}
            <a href="#">{"Some link"}</a>
        </div>
    }
}
fn something_child() -> Html {
    html! {
        <div class="something-child">
            <h1>{"Some child"}</h1>
            {something_child_child()}
        </div>
    }
}
fn something_child_child() -> Html {
    html! {
        <div class="something-child-child">
            <h4>{"Final thing"}</h4>
        </div>
    }
}
```

## Pure Components/Functional Components

Pure components are components which don't mutate their state, only displaying content and propagating messages up to normal, mutable components. They differ from view functions in that they can be used from within the `html!` macro using the component syntax \(`<SomePureComponent />`\) instead of expression syntax \(`{some_view_function()}`\), and that depending on their implementation, they can be memoized - preventing re-renders for identical props using the aforementioned `neq_assign` logic.

Yew doesn't natively support pure or functional components, but they are available via external crates.

Functional components don't exist yet, but in theory, pure components could be generated by using proc macros and annotating functions.

## Keyed DOM nodes when they arrive

## Compile speed optimizations using Cargo Workspaces

Arguably, the greatest drawback to using Yew are the long compile times. Compile time seems to correlate with the quantity of code found within `html!` macro blocks. For smaller projects this is unlikely to be a problem, but for larger webapps which span multiple pages, it makes sense to break apart your code across multiple crates to minimize the amount of work the compiler has to do.

You should try to make your main crate handle routing/page selection, move all commonly shared code to another crate, and then make a different crate for each page, where each page could be a different component, or just a big function that produces `Html`. In the best case scenario, you go from rebuilding all of your code on each compile to rebuilding only the main crate, and one of your page crates. In the worst case, where you edit something in the "common" crate, you will be right back to where you started: compiling all code that depends on that commonly shared crate, which is probably everything else.

If your main crate is too heavyweight, or you want to rapidly iterate on a deeply nested page \(eg. a page that renders on top of another page\), you can use an example crate to create a more simple implementation of the main page and render your work-in-progress component on top of that.

## Build size optimization

- optimize Rust code
  - `wee_aloc` ( using tiny allocator )
  - `cargo.toml` ( defining release profile )
- optimize wasm code using `wasm-opt`

More information about code size profiling: [rustwasm book](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size)


### wee\_alloc

[wee\_alloc](https://github.com/rustwasm/wee_alloc) is a tiny allocator that is much smaller than the allocator that is normally used in Rust binaries. Replacing the default allocator with this one will result in smaller WASM file sizes, at the expense of speed and memory overhead.

The slower speed and memory overhead are minor in comparison to the size gains made by not including the default allocator. This smaller file size means that your page will load faster, and so it is generally recommended that you use this allocator over the default, unless your app is doing some allocation-heavy work.

```rust
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### Cargo.toml

It is possible to setup release build for smaller size using `[profile.release]` section in `Cargo.toml`

[Rust profiles documentation](https://doc.rust-lang.org/cargo/reference/profiles.html)

```toml
[profile.release]
# less code to include into binary
panic = 'abort' 
# optimization over all codebase ( better optimization, slower build )
codegen-units = 1
# optimization for size ( more aggresive )
opt-level = 'z' 
# optimization for size 
# opt-level = 's' 
# link time optimization using using whole-program analysis
lto = true 
```

### wasm-opt

Further more it is possible to reduce the size of Wasm code.

wasm-opt info: [binaryen project](https://github.com/WebAssembly/binaryen)

The Rust Wasm book features a section about reducing the size of Wasm binaries: [Shrinking .wasm size](https://rustwasm.github.io/book/game-of-life/code-size.html)

- using `wasm-pack` which by default optimizes `wasm` code in release builds
- using `wasm-opt` directly on `wasm` files.

```
wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm
```

#### Build size of the 'minimal' example in `yew/examples`

Note: `wasm-pack` combines optimization for Rust and Wasm code. `wasm-bindgen` is in this example without any `Rust` size optimization.


| used tool                   | size 
| ---                         | ---
| wasm-bindgen                | 158KB  
| wasm-binggen + wasm-opt -Os | 116KB 
| wasm-pack                   | 99 KB
