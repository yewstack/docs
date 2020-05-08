# The Fetch Service
## Introduction
The fetch module can be used to make requests to a server. This allows a Yew application to connect to a backend and provide interactive functionality. 

## Making requests
### Building requests
Yew has a struct `Request` (which comes from the `http` crate) that is used to 'build' requests before they can be dispatched to a server. The body type of the request must have the trait `Into<Text>`.
```rust
use yew::services::fetch::Request;
use yew::format::Nothing;
let get_request = Request::get("https://example.com/api/v1/get/something").body(Nothing).expect("Could not build that request")
```
```rust
use yew::services::fetch::Request;
use yew::format::Json;
use serde_json::json;
let post_request = Request::post("https://example.com/api/v1/post/something").header("Content-Type", "application/json").body(Json(&json!({"key": "value"}))).expect("Could not build that request.")
```
### Dispatching requests
The `FetchService` provides a binding to the browser's `fetch` API. Requests can be sent using either `FetchService::fetch` or `FetchService::fetch_with_options` (`fetch_with_options` should be used where cookies need to be sent in a request).

`FetchService::fetch` accepts two parameters: a `Request` object and a `Callback`. The closure with which the callback is constructed must take a single parameter of type `Response`. `Response` accepts a type argument – the type must implement `From<Text>`.   

{% hint style="info" %}
If you keep getting an error saying that "the operation was aborted" or "Error 408" this might be because the [CORS headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) of the website you are trying to access are not set correctly.
{% endhint %}

```rust 
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask};
use serde::Deserialize;

#[derive(Debug, Clone)]
struct FetchServiceExample {
    ft: Option<FetchTask>,
    iss: Option<ISS>,
    link: ComponentLink<Self>
}

impl FetchServiceExample {
    fn view_iss_location(&self) -> Html {
        match self.iss {
            Some(space_station) => html! {
                <p>{"The ISS is at:"}</p>
                <p>{format!("Latitude: {}", space_station.iss_location.latitude)}</p>
                <p>{format!("Longitude: {}", space_station.iss_location.longitude)}</p>
            }
            None => html! {
                <button onclick=self.link.callback(|_| {<Self as Component>::Message::GetLocation})>
                    {"Where is the ISS?"}
                </button>
           }
        }
    }
    fn is_fetching(&self) -> Html {
        if self.fetching {
            html! {<p>{"Fetching data..."}</p>}
        } else {
            html! {<p></p>}
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ISSPosition {
    latitude: String,
    longitude: String
}

#[derive(Deserialize, Debug, Clone)]
struct ISS {
    message: String,
    timestamp: i32,
    iss_position: ISSPosition,
    fetching: bool
}

#[derive(Debug, Clone)]
enum Msg {
    GetLocation,
    Noop,
    ReceiveLocation(ISS)
}

impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ft: None,
            iss: None,
            link,
            fetching: false
        }
    }
    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            GetLocation => {
                // 1. build the request
                let request = Request::get("http://api.open-notify.org/iss-now.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback = self.link.callback(|response: Response<Json<Result<ISS, anyhow::Error>>>| {
                    // split up the response into the HTTP data about the request result and data from the request
                    // Json(data) has type `Json<Result<ISS, anyhow::Error>>`
                    let (meta, Json(Ok(data))) = response.into_parts(); 
                    if meta.status.is_success() && data.message == "success" {
                        Self::Message::ReceiveLocation(match data {
                            Ok(d) => d,
                            // handle errors more properly than this
                            Err(_) => panic!("Could not handle this error")
                        })
                    } else {
                        Self::Message::Noop
                    }
                });
                // 3. pass the request and callback to the fetch service 
                FetchService::new().fetch(request, callback);
                self.fetching = true;
                // we want to redraw so that the page displays a 'fetching...' message to the user so return 'true'
                true
            }
            ReceiveLocation(location) => {
                self.iss = location;
                self.fetching = false;
                // we want to redraw so that the page displays the location of the ISS instead of 'fetching...'
                true
            }
            _ => false
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                {self.is_fetching()}
                {self.view_iss_location()}
            </>
        }
    }
}
```

## Further reading
* [The API documentation](https://docs.rs/yew/0.14.3/yew/services/fetch/index.html)
* The [dashboard](https://github.com/yewstack/yew/tree/master/examples/dashboard) and [npm_and_rest](https://github.com/yewstack/yew/tree/master/examples/web_sys/npm_and_rest) examples.
