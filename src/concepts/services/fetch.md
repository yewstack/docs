# Fetch
## Introduction
The format module can be used to make requests to a server. This allows a Yew application to connect to a backend. 

## Making requests
### Building requests
Before you can send a request to the server, you need to construct it. This should be done using the `yew::services::fetch::Request` object.
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

## Further reading
* [The API documentation](https://docs.rs/yew/0.14.3/yew/services/fetch/index.html)
