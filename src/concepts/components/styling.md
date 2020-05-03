# Styling

&lt;Work in progress&gt;

Proposal for proper CSS support can be found here: [https://github.com/yewstack/yew/issues/533](https://github.com/yewstack/yew/issues/533)


In the meantime you can define the style with any standard CSS tools

## Yew component does not support yet class attribute

So far Yew component does not support natively the `class` attribute. You can add the class attribute to your component:

```rust
#[derive(Properties, Clone, PartialEq)]
...
pub struct Props{
...
#[prop_or_default]
    pub class:String,
...
}
...
```

and render how you wish!

```rust
...
        html! {
            <div class={format!("{}",self.props.class)}>
                <h1>{"I am super Class"}</h1>
            </div>
        }
...
```
