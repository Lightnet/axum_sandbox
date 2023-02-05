 * https://www.youtube.com/watch?v=zWNKQq1HSdg

```rust
let greeting_file_result = File::open("hello.tet");

let greeting_file = match greeting_file_result{
  Ok(file) =>file,
  Err(error) => panic!("Problem opening the file {:?}",error),
}
```