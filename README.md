 
# axum_sandbox

# crate packages:
 * axum 0.6.4
 * tokio 1.25
 * serde 1
 * serde_json 1
 * http 0.2
 * tower 0.4.13
 * tower-http 0.3.4
 * tracing 0.1
 * tracing-subscriber 0.3.16

# Information:
  web http server build tests.

.env
```
#
DATABASE_URL="postgres://user:password@127.0.0.1/test"
```

# Database:
  * sqlx
  * Postgres

# watch:

```
cargo install cargo-watch
```

```
cargo watch -q -c -x 'run -q'
```


# Refs:
 * https://docs.rs/axum/latest/axum/