---
description: A component can maintain its own state and render information depending on it
---

# Internal state

The component can manage it's own state using a struct that implement the trait `Component`. The HTML rendering is based on this state.
When the state change the component might be re-rendered.

```rust
use yew::prelude::*;

pub struct InternalStateComponent {
    name:String,
}

impl Component for InternalStateComponent {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            name: "Clark".into(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{format!("Hello {}",self.name)}</h1>
            </>
        }
    }
}
```

## Defining state

Here we add the `name` field to the struct

```rust
// ...
pub struct InternalStateComponent {
    name:String,
}
// ...
```

## State initialization

The component lifecycle will initialize the state in the `create` method.

```rust
// ...
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            name: "Clark".into(),
        }
    }
// ...
```

## Rendering using the state

Using the `html!` macro we can render html using the state from the `view` method

> please refer to the `html!` macro documentation page for more detail on how to render components as HTML

```rust
// ...
    fn view(&self) -> Html {
        html! {
            <h1>{format!("Hello {}", self.name)}</h1>
        }
    }
// ...
# }
```
