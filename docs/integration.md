# Integration

## SDK Download Link

```html
SDK Download Link: ​https://github.com/blotoutio/sdk-rust
Rust SDK C++ Wrapper: https://github.com/blotoutio/sdk-rust-ffi
```

## Initialization

```html

Initialize sdk with data that we generated in Application section on Blotout Dashboard.
bo_sdk_init(
    TOKEN.to_string(),
    END_POINT.to_string(),
    BUNDLE_ID.to_string(),
)
.await;

```

