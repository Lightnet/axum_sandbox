https://www.propelauth.com/post/clean-code-with-rust-and-axum



https://github.com/tokio-rs/axum/pull/1543
```rust
#[derive(Clone)]
struct AppState {
    count: u64,
}

pub fn api() -> RouterService {
    Router::<AppState>::new()
        .nest("/api/v1", api_v1())
        //               ^^^^^^^^ expected struct `AppState`, found `()`
        .fallback(fallback)
        .with_state(AppState { count: 0 })
}

fn api_v1() -> Router {
    Router::new()
        .route("/teams", get(get_teams))
        .route("/teams/:slug", get(get_team))
}
```