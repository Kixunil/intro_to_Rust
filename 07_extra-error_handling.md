Error handling (extra)
======================

Rust defines simple type used for error handling in the core library:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Obviously, function that can fail could look like this:

```rust
fn open_file(file_name: &str) -> Result<File, OpenError> {
    // ...
}
```

And it can be used like this:

```rust
let file_name = "hello_world.rs";

match open_file(file_name) {
    Ok(file) => file.read(),
    Err(error) => println!("Error opening file {}: {}", file_name, error),
}
```

But this is very common pattern:
```rust
let file = match open_file(file_name) {
    Ok(file) => file, // See that we don't have to write Result::Ok? Rust already brought us Ok in scope.
                      // We could do the same thing with `use MyResult::{MyOk, MyErr};` or simply `use MyResult::*;`
    Err(error) => return error.into(), // into is conversion method from `Into` trait.
}
```

To avoid such boilerplate, `try!()` macro was invented:

```rust
let file = try!(open_file(file_name)); // This does the exact same thing as the code above
```

Which led to adding special question mark operator `?`:

```rust
let file = open_file(file_name)?; // This does the exact same thing as the code above
```

This is nice because when reading, one sees operation first and worries about error handling second. Also this:
```rust
let value = open_file(file_name)?.read_all()?.deserialize()?;
```

is much nicer than this:
```rust
let value = try!(try!(try!(open_file(file_name)).read_all()).deserialize());
```

Pros of Rust error handling:
* No costly unwinding
* You immediatelly see where a function could "throw" error
* Not much of extra syntax like `try`, `catch`, `throw`... Just simple enums and `?`

Now, when we learned the most important differences, let's try to create something!
