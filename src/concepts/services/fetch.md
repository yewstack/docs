# Fetch
## Introduction
The fetch module can be used to make requests to a server. This allows a Yew application to connect to a backend. 

## Making requests
### Building requests
Before you can send a request to the server, you need to construct the request. This should be done using the `yew::services::fetch::Request` object.
```rust
use yew::services::fetch::Request;
use yew::format::Nothing;
let get_request = Request::get("https://example.com/api/v1/something").body(Nothing).expect("Could not build that request")
```

```rust
use yew::services::fetch::Request;
use yew::format::Json;
use serde_json::json;
let post_request = Request::post("https://example.com/api/v1/post/something").header("Content-Type", "application/json").body(Json(&json!({"key": "value"}))).expect("Could not build that request.")
```
### Dispatching requests
To dispatch requests an instance of the `fetch_service` is needed. This can be created using `yew::services::FetchService::new()`. After building a request, it can be dispatched using `FetchService`'s `fetch` method. This method takes two arguments: a request and a `ComponentLink` callback with a closure taking an instance of `yew::services::fetch::Response`. This closure is run after the request finishes and can be used to initiatiate a redraw of the component by returning a message.

The `Response` type takes a generic `Result` as a type argument. You should specify types for your callback closure; any type can be used as long as it implements `From<Text>` (`Text` is a type from the format module), this can be achieved by deriving serde's `Deserialize`. 

An example closure which could be used:
```rust
|response: Response<Result<CustomType, Error>>| -> Msg {
    match response {
        Ok(response) => {
            // handle response here
        }
        Err(e) => {
            // handle errors here
        }
    }
}
```

## Further reading
* [The API documentation](https://docs.rs/yew/0.14.3/yew/services/fetch/index.html)
* The [dashboard](https://github.com/yewstack/yew/tree/master/examples/dashboard) and [npm_and_rest](https://github.com/yewstack/yew/tree/master/examples/web_sys/npm_and_rest) examples.
