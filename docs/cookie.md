 
 
 * https://github.com/imbolc/tower-cookies/blob/main/examples/counter.rs
 
 
 
 
 
 
 
 * https://github.com/tokio-rs/axum/discussions/351
 * https://github.com/imbolc/tower-cookies
 * https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/struct.SignedCookieJar.html
 * 
 * https://docs.rs/tower-cookies/latest/tower_cookies/
 * https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/struct.SignedCookieJar.html
 * 
 * 
 * https://www.shuttle.rs/blog/2022/08/11/authentication-tutorial
 * https://github.com/imbolc/tower-cookies
 * 
 * 
 * 




```rust
use axum::{
    http::header::SET_COOKIE,
    response::{Headers, Html, IntoResponse},
};

async fn handler() -> impl IntoResponse {
    let headers = Headers([(SET_COOKIE, "key=value")]);
    let content = Html("<h1>Hello, World!</h1>");
    (headers, content)
}
```