---
description: Components and their lifecycle hooks
---

# Components

## What are Components

Components are the building blocks of Yew. They manage their own state and can render themselves to the DOM. Components are created by implementing the `Component` trait which describes the lifecycle of a component.

## Example of component

```rust
use yew::prelude::*;

pub struct ExampleComponent {
    // state of the component
    name: String,
    show_message: bool,

    // properties and events bag
    props: Props,

    // link field supports the mechanism through which components are able to register callbacks and update themselves
    link: ComponentLink<Self>,
}

// enum of "Messages" that will be used to mutate the component state
pub enum Msg {
    Click(),
}

// definition of properties and events of the component
#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Superman".to_string())]
    pub name: String,

    #[prop_or_default]
    pub onmyclickevent:Callback<String>,
}

impl Component for ExampleComponent {
    type Message = Msg;
    type Properties = Props;

    // Initialization of the state
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props.clone(),
            name: props.name.into(),
            show_message: false,
        }
    }

    // This method is executed each time the link.callbacks is called 
    // you can mutate the state based on the message received
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

            // mutate the component state
            Msg::Click() => self.show_message = true,
        }
        true
    }

    // mutate state if the properties bag is changed
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    // Rendering of the component
    // The method is called when 'ShouldRender' is 'true' as return of update and change method
    fn view(&self) -> Html {
        // different rendering depend on the component state
        if !self.show_message {
            html! {
                // trap HTML events and trigger a message that will be managed in the update method
                <button onclick=self.link.callback( |_| Msg::Click() )>{"Click to say hello!"}</button>
            }
        } else {
            html! {
                // Use state value in the html
                <h1>{format!("Hello {}",self.name)}</h1>
            }
        }
    }
}
```