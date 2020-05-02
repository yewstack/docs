# Format

## Introduction
Yew provides the format module to make it easy to convert from Rust types to common data formats (and vice versa). Rust provides support for a number of different data formats. Some of these formats are binary-only (e.g. CBOR) while others support both text and binary (e.g. JSON). The format module isn't useful in and of itself, but is necessary for using other services (such as the WebSocket and Fetch services). 

The format module has two core types which can be converted to: `Text` and `Binary`. All types which can be converted into `Text` can also be converted into `Binary` (but not vice-versa). 

## Example usage
```rust
use yew::format::Json;
let data = &"{\"key\": \"value\"}".to_string();
// convert the data into JSON (lazily)
let json_data = Json(&data);
// convert the JSON back
let Json(dump) = json_data;
```
Conversion is done lazily, i.e. data is only converted into text/binary data when you pass a value to a function which uses the values, such as `FetchService`'s `FetchService::fetch` function.

## Custom types
You can use the format module to convert custom types. In order to do this, the types you try to convert must implement the `Serialize` trait from the `serde` module if you wish to convert them into raw data and `Deserialize` if you wish to convert them from raw data.

```rust
use yew::format::{Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct APIRequest {
    api_token: String,
    api_data: i32
}

let new_request = APIRequest {
    api_token: String::from("demo token"),
    api_data: 15
}
// convert the custom data type into JSON text – note that `.into()` had to be called before the data was converted
let raw_api_request: Text = Json(&new_request).into();
// convert the JSON back into our custom data type.
let Json(rebuilt_request): Json<Result<APIRequest, _>> = Json::from(Ok(raw_api_request.unwrap()));
```

## Further reading
* [The API documentation](https://docs.rs/yew/*/yew/format/index.html)
* [The serde documentation](https://serde.rs)
