# Integration

SDK Download Link: [https://github.com/blotoutio/sdk-rust](https://github.com/blotoutio/sdk-rust)

Rust SDK C++ Wrapper: [https://github.com/blotoutio/sdk-rust-ffi](https://github.com/blotoutio/sdk-rust-ffi)

## Initialization

```rust
use blotout::bo_init;

const TOKEN: &str = "7T3VGKRTMZND4Q9";
const ENDPOINT_URL: &str = "https://domain.com/sdk";

bo_init(TOKEN.to_string(), ENDPOINT_URL.to_string()).await;
```
