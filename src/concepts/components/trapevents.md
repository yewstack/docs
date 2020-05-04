---
description: Component could mutate internal state or re-render based on events emitted by html or yew components
---

# Trap events

The framework provide the capability to update the internal state, for example, when an event is emitted by a child component.

The `update` method could be called and mutate the internal state. The `update` method is called via `self.link.callback`, `link` being an attribute of the component struct.

The `update` method receives "context" by the argument `msg` of type `Self::Message`. You can define any type for `Message`. The common way is to define an enum `Msg` for any action that can mutate the state. Then define `Msg` as the type of `Message` in the Component trait implementation.

At the end of the `update` method, you can decide to render the component returning `true`.

```rust
use yew::prelude::*;

pub struct TrapEventComponent {
    link: ComponentLink<Self>,

    name: String,
    show_message: bool,
}

pub enum Msg {
    Click(),
}

impl Component for TrapEventComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: "Clark".into(),
            show_message: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click() => self.show_message = true,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if !self.show_message {
            html! {
                <>
                    <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"}</button>
                </>
            }
        } else {
            html! {
                <>
                    <h1>{format!("Hello {}",self.name)}</h1>
                </>
            }
        }
    }
}
```

## Define the `link` attribute in the state

```rust
# use yew::prelude::*;
// ...
 pub struct TrapEventComponent {
     link: ComponentLink<Self>,

// ...
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }
#
# impl Component for TrapEventComponent {
#     type Message = Msg;
#     type Properties = ();
#
#     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             name: "Clark".into(),
#             show_message: false,
#         }
#     }
#
#     fn update(&mut self, msg: Self::Message) -> ShouldRender {
#         match msg {
#             Msg::Click() => self.show_message = true,
#         }
#         true
#     }
#
#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
#     fn view(&self) -> Html {
#         if !self.show_message {
#             html! {
#                 <>
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"}</button>
#                 </>
#             }
#         } else {
#             html! {
#                 <>
#                     <h1>{format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }
```

## Define a Message enum

```rust
# use yew::prelude::*;
#
# pub struct TrapEventComponent {
#     link: ComponentLink<Self>,
#
#     name: String,
#     show_message: bool,
# }

// ...
pub enum Msg {
    Click(),
}

impl Component for TrapEventComponent {
    type Message = Msg;
    type Properties = ();

// ...
#     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             name: "Clark".into(),
#             show_message: false,
#         }
#     }
#
#     fn update(&mut self, msg: Self::Message) -> ShouldRender {
#         match msg {
#             Msg::Click() => self.show_message = true,
#         }
#         true
#     }
#
#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
#     fn view(&self) -> Html {
#         if !self.show_message {
#             html! {
#                 <>
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"}</button>
#                 </>
#             }
#         } else {
#             html! {
#                 <>
#                     <h1>{format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }





```

## Update the internal state based on the context

```rust
# use yew::prelude::*;
#
# pub struct TrapEventComponent {
#     link: ComponentLink<Self>,
#
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }
#
# impl Component for TrapEventComponent {
#     type Message = Msg;
#     type Properties = ();
#
#     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             name: "Clark".into(),
#             show_message: false,
#         }
#     }

// ...
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click() => self.show_message = true,
        }
        true
    }
// ...

#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
#     fn view(&self) -> Html {
#         if !self.show_message {
#             html! {
#                 <>
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"}</button>
#                 </>
#             }
#         } else {
#             html! {
#                 <>
#                     <h1>{format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }

```

## Trap the html native events

```rust
# use yew::prelude::*;
#
# pub struct TrapEventComponent {
#     link: ComponentLink<Self>,
#
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }
#
# impl Component for TrapEventComponent {
#     type Message = Msg;
#     type Properties = ();
#
#     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             name: "Clark".into(),
#             show_message: false,
#         }
#     }
#
#     fn update(&mut self, msg: Self::Message) -> ShouldRender {
#         match msg {
#             Msg::Click() => self.show_message = true,
#         }
#         true
#     }
#
#     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
#         true
#     }
#
#     fn view(&self) -> Html {
#         if !self.show_message {
// ...
            html! {
                <>
                    <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"}</button>
                </>
// ...
#             }
#         } else {
#             html! {
#                 <>
#                     <h1>{format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }
```

**TODO: extend adding the treatment based on an event state or redirect to example**
