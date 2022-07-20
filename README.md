# coreemu-rs

This is a client library to use the gRPC interface of [coreemu](https://github.com/coreemu/core) from rust.
All gRPC functions are exposed in their raw form as produced by [tonic-build](https://crates.io/crates/tonic-build) but there are is also a thin (and incomplete) wrapper to make the API a bit more usable.

The main branch of this repository and the version published on crates.io is build for coreemu 7.5.2, there is a branch coreemu8 which has support for the changed gRPC interface.

To use this library add the following dependency to your `Cargo.toml`:

```
coreemu = "*"
```

or just call:
```
cargo add coreemu
```


## Example

The following example connects to the local *coreemu* instance and prints all node information from the first running instance found.

```rust
use coreemu::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("http://127.0.0.1:50051").await?;

    let response = client.get_sessions().await?;

    println!("RESPONSE={:?}", response);

    let session_id = response[0].id;

    let response = client.get_session(session_id).await?.unwrap();

    for n in response.nodes {
        if n.name.starts_with("n") {
            println!("RESPONSE={:#?}", n.position.unwrap());
        }
    }
    Ok(())
}
```