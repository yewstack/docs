---
description: Component can maintain it's own state and render information depending on it
---

# Internal State

The component can manage it's own state withing Rust struct. The rendering is done based on the state and it's mutation.

> **TODO a chapter state mutation**

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
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
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

## State definition

Here we add the `name` field in the struct

```rust
# use yew::prelude::*;
#
// ...
pub struct InternalStateComponent {
    name:String,
}
// ...
#
# impl Component for InternalStateComponent {
#     type Message = ();
#     type Properties = ();
#
#     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
#         Self {
#             name: "Clark".into(),
#         }
#     }
#
#     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
#         true
#     }
#
#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
#     fn view(&self) -> Html {
#         html! {
#             <>
#                 <h1>{format!("Hello {}",self.name)}</h1>
#             </>
#         }
#     }
# }

```

## State initialization

The component lifecycle will initialize the state in the `create` method.

```rust
# use yew::prelude::*;
#
# pub struct InternalStateComponent {
#     name:String,
# }
#
# impl Component for InternalStateComponent {
#     type Message = ();
#     type Properties = ();
// ...
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            name: "Clark".into(),
        }
    }
// ...
#     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
#         true
#     }
#
#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
#     fn view(&self) -> Html {
#         html! {
#             <>
#                 <h1>{format!("Hello {}",self.name)}</h1>
#             </>
#         }
#     }
# }
```

## Rendering based on the state value

Using the `html!` macro we can render html based on the component state in the `view` method

> please refer to the `html!` macro documentation page for more detail on the HTML rendering

```rust
# use yew::prelude::*;
#
# pub struct InternalStateComponent {
#     name:String,
# }
#
# impl Component for InternalStateComponent {
#     type Message = ();
#     type Properties = ();
#
#     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
#         Self {
#             name: "Clark".into(),
#         }
#     }
#
#     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
#         true
#     }
#
#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
// ...
    fn view(&self) -> Html {
        html! {
            <>
                <h1>{format!("Hello {}",self.name)}</h1>
            </>
        }
    }
// ...
# }
```
