# Introduction

## What is Yew?

**Yew** is a Rust framework for creating multi-threaded frontend web apps with WebAssembly.

#### Why Rust and WebAssembly?

\*\*\*\*[**Rust**](https://www.rust-lang.org/) is blazing fast while at the same time being super reliable with its rich type system and ownership model. It can have a tough learning curve but is worth the effort. Rust has been voted the most loved programming language [multiple](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) [years](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) in a row in Stack Overflow Developer Surveys. 

\*\*\*\*[**WebAssembly**](https://webassembly.org/) _\(Wasm\)_ is a portable low-level language that Rust can compile into which aims to run at native speeds in the browser and is interoperable with JavaScript and supported in all major browsers.

### Modern Web Framework

Yew is a component-based framework that makes it simple and painless to create complex interactive UIs. Developers who have experience with frameworks like React and Elm, should feel quite at home when using Yew. Creating HTML in Yew even looks a lot like React's JSX with a few minor exceptions. Here's a quick look:

```rust
fn view(&self) -> Html<Self> {
  html! {
    <section class="todoapp">
      <header class="header">
        <h1>{ "todos" }</h1>
      </header>
      <section class="main">
        <input type="checkbox"
            checked=self.all_completed()
            onclick=|_| Msg::ToggleAll />
        { self.view_todos() }
      </section>
    </section>
  }
}
```

### Concurrency and Performance

First and foremost, it should be clear that using Wasm is not a silver bullet for improving performance of a web app. As of the time of writing, using DOM APIs from a WebAssembly execution environment is slower than calling from JavaScript. This is because until  

 is not going to be faster than a JavaScript app if you're primarily using DOM APIs. This will probably change in the near future, with the adoption of [Web IDL](https://heycam.github.io/webidl/). But for the time being, Wasm applications have to serialize commands from Wasm to JavaScript to interact with the DOM which will impact performance. Yew minimizes this serialization overhead by internally maintaining a "virtual DOM" to send the minimal amount of patches needed to update the browser page for each re-render of your app.





 built before the standardization of async-await and has promoted the use of the [actor model](https://en.wikipedia.org/wiki/Actor_model) of concurrency. This model will feel very natural if you choose to write the server side of your application with [actix-web](https://github.com/actix/actix-web). In Yew, an actor is called an `Agent`. Using agents, a Yew application can delegate tasks to worker threads using Web Workers and subscribe to async messages from those agents.

An alternative approach is using futures which are can be leveraged through the [wasm-bindgen-futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/) crate which bridges Rust futures to JS Promises. An example project using futures and async-await can be found [here](https://github.com/yewstack/yew/tree/master/examples/futures).

### 



First of all, using WASM is not going to be faster than a JavaScript app if you're primarily using DOM APIs. This will probably change in the near future, with the adoption of [Web IDL](https://heycam.github.io/webidl/). But for the time being, Wasm applications have to serialize commands from Wasm to JavaScript to interact with the DOM which will impact performance. Yew minimizes this serialization overhead by internally maintaining a "virtual DOM" to send the minimal amount of patches needed to update the browser page for each re-render of your app.

That being said, WebAssembly can be leveraged for data heavy and graphics intensive calculations in the background. When client UI performance is not too important \(internal tooling, for example\) using WebAssembly for the full web application can be acceptable.

### Safety

Rust helps developers write safer code. For example, in JavaScript, an uncaught error can cause serious problems in your application. Rust encourages proper error handling and you can even get stacktraces for your Rust code with the [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) crate. Also, Yew apps can leverage Rust's static typing to ensure that your `Component` receives the correct properties for creation \(otherwise your app won't compile!\).

### JavaScript Interop

Yew is built on top of great web tooling like `wasm-bindgen` and `stdweb` and will be supporting `web-sys` and `js-sys` in the near future. These crates enable WebAssembly code to call into JavaScript and vice-versa. For some examples, visit [here](https://github.com/yewstack/yew/tree/master/examples/js_callback) to see how to leverage `stdweb` to write JavaScript code in your Rust app and [here](https://github.com/yewstack/yew/tree/master/examples/npm_and_rest) for how to interact with an NPM module.

### Declarative User Interfaces

Yew includes a procedural macro for generating HTML. It closely resembles React's JSX with some exceptions _\(string literals and listener callbacks to name a few\)_. Here is a quick look at its usage:



```rust
Be sure to check out the html! Guide for more information.
```

### 

