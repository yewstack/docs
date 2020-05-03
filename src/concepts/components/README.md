---
description: Components and their lifecycle hooks
---

# Components

## What are Components?

Components are the building blocks of Yew. They manage their own state and can render themselves to the DOM. Components are created by implementing the `Component` trait which describes the lifecycle of a component.

## Simple Component

```rust
use yew::prelude::*;
pub struct SimpleComponent {}

impl Component for SimpleComponent {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Hello World!"}</h1>
            </>
        }
    }
}
```

> one think to highlight here, is how to render the message. As explained in the `html!` macro doumentation, the literals must be quoted and curly braced: `{"Hello World!"}`

This component could be invoked in your page like this:

```rust
    ...
        <SimpleComponent />
    ...
```