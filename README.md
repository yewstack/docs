# 简介

## What is Yew?

**Yew** is a modern [Rust](https://www.rust-lang.org/) framework for creating multi-threaded front-end web apps with [WebAssembly](https://webassembly.org/).

* It features a **component-based** framework which makes it easy to create interactive UIs. Developers who have experience with frameworks like [React](https://reactjs.org/) and [Elm](https://elm-lang.org/) should feel quite at home when using Yew.
* It has **great performance** by minimizing DOM API calls by helping developers easily offload processing to background web workers. 
* It supports **JavaScript interoperability**, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.

### Join Us 😊

* You can report bugs and discuss features on the [GitHub issues page](https://github.com/yewstack/yew/issues)
* We ❤️pull requests. Check out [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) if you'd like to help out!
* Our [Gitter chatroom](https://gitter.im/yewframework/Lobby) is pretty active and is a great place to ask questions

![Our community is thriving!](https://img.shields.io/github/stars/yewstack/yew?color=009A5B&label=Github%20stars)

### Ready to dive in?

Click the link below to learn how to build your first Yew app and learn from community example projects

{% page-ref page="cong-ling-kai-shi/getting-started.md" %}

### \*\*\*\*

### **Still not convinced?**

This project is built on cutting edge technology and is great for developers who like to develop the foundational projects of tomorrow. Here are some reasons why we believe that frameworks like Yew are the future of web development.

#### **Wait, why WebAssembly?**

WebAssembly _\(Wasm\)_ is a portable low-level language that Rust can compile into. It runs at native speeds in the browser and is interoperable with JavaScript and supported in all major browsers. For more ideas on how to get the most out of WebAssembly, check out this list of [Use Cases](https://webassembly.org/docs/use-cases/).

It should be noted that using Wasm is not \(yet\) a silver bullet for improving the performance of a web app. As of right now, using DOM APIs from WebAssembly is still slower than calling them directly from JavaScript. This is a temporary hurdle which the [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) proposal aims to resolve. If you would like to learn more, check out this [excellent article](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) from Mozilla.

#### Ok, but why Rust?

Rust is blazing fast and reliable with its rich type system and ownership model. It has a tough learning curve but is well worth the effort. Rust has been voted the most loved programming language in Stack Overflow's Developer Surveys for both [2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) and [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages).

Rust also helps developers write safer code with its rich type system and ownership model. Say goodbye to hard to track down race condition bugs in JavaScript! In fact, with Rust, most of your bugs will be caught by the compiler before your app even runs. And don't worry, when your app does run into an error, you can still get full stack-traces for your Rust code in the browser console.

#### Alternatives?

We love to share ideas with other projects and believe we can all help each other reach the full potential of this exciting new technology. If you're not into Yew, you may like the following projects \(listed alphabetically\)

* [Draco](https://github.com/utkarshkukreti/draco) - _"A Rust library for building client side web applications with Web Assembly"_
* [Percy](https://github.com/chinedufn/percy) - _"A modular toolkit for building isomorphic web apps with Rust + WebAssembly"_
* [Seed](https://github.com/seed-rs/seed) - _"A Rust framework for creating web apps"_
* [Smithy](https://github.com/rbalicki2/smithy) - _"A framework for building WebAssembly apps in Rust"_

