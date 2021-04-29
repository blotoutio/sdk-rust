# Integration

SDK Download Link: https://github.com/blotoutio/sdk-rust

Rust SDK C++ Wrapper: https://github.com/blotoutio/sdk-rust-ffi

## Initialization

```rust
const token: &str = "7T3VGKRTMZND4Q9";
const endpoint_url: &str = "https:/domain.com/sdk";

bo_init(
    token.to_string(),
    endpoint_url.to_string(),
)
.await;
```
