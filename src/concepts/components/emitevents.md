---
description: Component could share update to the parent html component emitting events
---

# Emit events

It's possible to update the parent html component emitting **events** in the same way we received data via **properties**.

The events are defined as part of the `Properties` struct.

When it's needed you just need to call the `emit()` method of the event, to propagate the update to the parents containers.

> You can name the event however you want to. It's usually started by "on"

```rust
use yew::prelude::*;

pub struct EmitEventComponent {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
    show_message: bool,
}

pub enum Msg {
    Click(),
    Click4Event(),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Clark by default".to_string())]
    pub name: String,

    #[prop_or_default]
    pub onmyclickevent:Callback<String>,
}

impl Component for EmitEventComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props.clone(),
            name: props.name.into(),
            show_message: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click() => self.show_message = true,
            Msg::Click4Event() => self.props.onmyclickevent.emit("Hello Loise".into()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
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
                    {"Click on clark to raised an event for the parent container ;-)"}
                    <h1 onclick=self.link.callback( |_| Msg::Click4Event()) >
                    {format!("Hello {}",self.name)}</h1>
                </>
            }
        }
    }
}
```

## Define the Event

Here we are defining a new event named `onmyclickevent` with a `String` as parameter.

```rust
# use yew::prelude::*;
#
# pub struct EmitEventComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
#     Click4Event(),
# }

// ...
#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Clark by default".to_string())]
    pub name: String,
    #[prop_or_default]
    pub onmyclickevent:Callback<String>,
}
// ...

# impl Component for EmitEventComponent {
#     type Message = Msg;
#     type Properties = Props;
#
#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             props: props.clone(),
#             name: props.name.into(),
#             show_message: false,
#         }
#     }
#
#     fn update(&mut self, msg: Self::Message) -> ShouldRender {
#         match msg {
#             Msg::Click() => self.show_message = true,
#             Msg::Click4Event() => self.props.onmyclickevent.emit("Hello Loise".into()),
#         }
#         true
#     }
#
#     fn change(&mut self, props: Self::Properties) -> ShouldRender {
#         if self.props != props {
#             self.props = props;
#             true
#         } else {
#             false
#         }
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
#                     {"Click on clark to raise an event for the parent container ;-)"}
#                     <h1 onclick=self.link.callback( |_| Msg::Click4Event()) >
#                     {format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }

```

## Emit the Event

The event are usually emitted in the `update()` method as effect of message

```rust
# use yew::prelude::*;
#
# pub struct EmitEventComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
#     Click4Event(),
# }
#
# #[derive(Properties, Clone, PartialEq)]
# pub struct Props{
#     #[prop_or("Clark by default".to_string())]
#     pub name: String,
#
#     #[prop_or_default]
#     pub onmyclickevent:Callback<String>,
# }
#
# impl Component for EmitEventComponent {
#     type Message = Msg;
#     type Properties = Props;
#
#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             props: props.clone(),
#             name: props.name.into(),
#             show_message: false,
#         }
#     }
// ...
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click() => self.show_message = true,
            Msg::Click4Event() => self.props.onmyclickevent.emit("Hello Loise".into()),
        }
        true
    }
// ...
#     fn change(&mut self, props: Self::Properties) -> ShouldRender {
#         if self.props != props {
#             self.props = props;
#             true
#         } else {
#             false
#         }
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
#                     {"Click on clark to raised an event for the parent container ;-)"}
#                     <h1 onclick=self.link.callback( |_| Msg::Click4Event()) >
#                     {format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }
```

## Mandatory or optionally binded events

Like for properties you can define the events to be be mandatory or optionaly binded in the parent component.

This is defined providing or not attributes to the event definition.

```rust
# use yew::prelude::*;
#
# pub struct EmitEventComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
#     Click4Event(),
# }
// ...
#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Clark by default".to_string())]
    pub name: String,

    pub on_i_am_mandatory_event:Callback<()>,

    #[prop_or_default]
    pub on_i_am_optional_event:Callback<()>,
}
// ...
# impl Component for EmitEventComponent {
#     type Message = Msg;
#     type Properties = Props;
#
#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
#         Self {
#             link,
#             props: props.clone(),
#             name: props.name.into(),
#             show_message: false,
#         }
#     }
#
#     fn update(&mut self, msg: Self::Message) -> ShouldRender {
#         match msg {
#             Msg::Click() => self.show_message = true,
#             Msg::Click4Event() => self.props.on_i_am_mandatory_event.emit(()),
#         }
#         true
#     }
#
#     fn change(&mut self, props: Self::Properties) -> ShouldRender {
#         if self.props != props {
#             self.props = props;
#             true
#         } else {
#             false
#         }
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
#                     {"Click on clark to raised an event for the parent container ;-)"}
#                     <h1 onclick=self.link.callback( |_| Msg::Click4Event()) >
#                     {format!("Hello {}",self.name)}</h1>
#                 </>
#             }
#         }
#     }
# }

```

Like for the properties a compilation error will be raised in case you omitted to bind a mandatory event. The message could be something like:

> no method named `build` found for struct `components::comp4::PropsBuilder<...PropsBuilderStep_missing_required_prop_name>` in the current scope
> method not found in `...::PropsBuilder<...PropsBuilderStep_missing_required_prop_name>`rustc(E0599)
comp4.rs(14, 10): method `build` not found for this`
