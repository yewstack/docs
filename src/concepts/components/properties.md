---
description: The component could expose property attributes to receive data from the parent html component
---

# Properties

It is a common need to receive data from the parent component using properties.

The properties are defined in a dedicated Rust struct derivating `Properties` and `Clone`traits.

> This struct is usually named `Props`

The properties bag is attached to the state adding the field `props` in the component struct and defining the `Properties` `Type` in the component trait implementation.

The properties have to be intialized in the `create` method. As for `link`, the framework push to the method the bag of the properties and can be used to attach it to the internal state.

It' possible to define some attributes to the properties as:

- optional and initialized with Rust default value
- optional and initialieed with component default value
- mandatory, the parent must define a value for the attribute

> remark: for convenience and/or performance optimization `Props` can derive `PartialEq` to avoid to re-render if there is no changed.cf. `change` method

```rust
use yew::prelude::*;

pub struct UseOfPropertyComponent {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
    show_message: bool,
}

pub enum Msg {
    Click(),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub name: String,
}

impl Component for UseOfPropertyComponent {
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
                    <h1>{format!("Hello {}",self.name)}</h1>
                </>
            }
        }
    }
}

```

In order to use this component you have to:

```html
// ...
    <div class="full-height">
        {"In this example we pass the name as parameter of the Yew component."}
        <UseOfPropertyComponent name="Clark"/>
    </div>
// ...

```

## Define the properties structure

```rust
# use yew::prelude::*;
#
# pub struct UseOfPropertyComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }

// ...
#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub name: String,
}
// ...

# impl Component for UseOfPropertyComponent {
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
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"} </button>
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

## Attach the property bag in the state

```rust
# use yew::prelude::*;
// ...
pub struct UseOfPropertyComponent {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
    show_message: bool,
}
// ...
#
# pub enum Msg {
#     Click(),
# }
#
# #[derive(Properties, Clone, PartialEq)]
# pub struct Props{
#     pub name: String,
# }
#
# impl Component for UseOfPropertyComponent {
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
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"} </button>
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

## Initialize the properties

```rust
# use yew::prelude::*;
#
# pub struct UseOfPropertyComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }
#
# #[derive(Properties, Clone, PartialEq)]
# pub struct Props{
#     pub name: String,
# }
#
# impl Component for UseOfPropertyComponent {
#     type Message = Msg;
#     type Properties = Props;
// ...
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props.clone(),
            name: props.name.into(),
            show_message: false,
        }
    }
// ...
#     fn update(&mut self, msg: Self::Message) -> ShouldRender {
#         match msg {
#             Msg::Click() => self.show_message = true,
#         }
#         true
#     }
#
#
#    fn change(&mut self, props: Self::Properties) -> ShouldRender {
#        if self.props != props {
#            self.props = props;
#            true
#        } else {
#            false
#        }
#    }
#
#     fn view(&self) -> Html {
#         if !self.show_message {
#             html! {
#                 <>
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"} </button>
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

> Here, to simply extend the previous example, we `.clone()` the `props` argument. It may not be needed in your code

## Mandatory property

In this example the property have to be defined using the component. If it's omitted a compilatioin error is raised. The error looks like:

> no method named `build` found for struct `components::comp4::PropsBuilder<...PropsBuilderStep_missing_required_prop_name>` in the current scope
> method not found in `...::PropsBuilder<...PropsBuilderStep_missing_required_prop_name>`rustc(E0599)
comp4.rs(14, 10): method `build` not found for this`

## Optional property

Property can be defined optional just adding `#[prop_or_default]` on the property. In that case the property value will be initialized by the default Rust type value.

```rust
# use yew::prelude::*;
#
# pub struct UseOfPropertyComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }

// ...
#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or_default]
    pub name: String,
}
// ...

# impl Component for UseOfPropertyComponent {
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
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"} </button>
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

In that case we will just say "Hello" ;-)

## Optional property with component default value

Property can be defined optional but adding a default component value adding `#[prop_or_(your_default_value)]` on the property.

```rust
# use yew::prelude::*;
#
# pub struct UseOfPropertyComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }
#
// ...
#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    #[prop_or("Clark by default".to_string())]
    pub name: String,
}
// ...
#
# impl Component for UseOfPropertyComponent {
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
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"} </button>
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

In that case we will say "Hello Clark by default" ;-)

## Optimize rendering in the `change` method

In order to avoid unecessary rendring it's possible to compare the mutation of the `props` bag in the `change` method.
This optimization imply to derive `PartialEq` for the `Props` struct to easily compare the `props` bag passed as argument of the method and the one in the internal state of the component.

```rust
# use yew::prelude::*;
#
# pub struct UseOfPropertyComponent {
#     link: ComponentLink<Self>,
#     props: Props,
#     name: String,
#     show_message: bool,
# }
#
# pub enum Msg {
#     Click(),
# }
#
# #[derive(Properties, Clone, PartialEq)]
# pub struct Props{
#     pub name: String,
# }
#
# impl Component for UseOfPropertyComponent {
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
#         }
#         true
#     }
// ...
     fn change(&mut self, props: Self::Properties) -> ShouldRender {
         if self.props != props {
             self.props = props;
             true
         } else {
             false
         }
     }
// ...

#     fn view(&self) -> Html {
#         if !self.show_message {
#             html! {
#                 <>
#                     <button onclick=self.link.callback( |_| Msg::Click() )>{"Click here!"} </button>
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

## Memory/speed overhead of using Properties

In `Component::view`, you take a reference to the component's state, and use that to create `Html`. Properties, however, are owned values. This means that in order to create them and pass them to child components, we need to take ownership of the references provided in the `view` function. This is done by implicitly cloning the references as they are passed to components in order to get owned values.

This means that each component has its own distinct copy of the state passed down from its parent, and that whenever you re-render a component, the props for all child components of the re-rendering component will have to be cloned.

The implication of this is if you would otherwise be passing _huge_ amounts of data down as props \(Strings that are 10s of kilobytes in size\), you may want to consider turning your child component into a function which returns `Html` that the parent calls, as this means that data does not have to be cloned.

If you won't need to modify the data passed down through props you can wrap it in an `Rc` so that only a reference-counted pointer to the data is cloned, instead of the actual data itself.

### Example

```rust
use std::rc::Rc;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

impl Default for LinkColor {
    fn default() -> Self {
        // The link color will be blue unless otherwise specified.
        LinkColor::Blue
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// The link must have a target.
    href: String,
    /// If the link text is huge, this will make copying the string much cheaper.
    /// This isn't usually recommended unless performance is known to be a problem.
    text: Rc<String>,
    /// Color of the link.
    #[prop_or_default]
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    #[prop_or_default]
    size: Option<u32>,
    /// When the view function doesn't specify active, it defaults to true.
    #[prop_or(true)]
    active: bool,
}
```
