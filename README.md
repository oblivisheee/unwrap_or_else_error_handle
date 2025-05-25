# unwrap_or_else_error_handle

A simple Rust utility to handle errors by printing a custom message and exiting the program.

## Usage

Add the function to your project:

```rust
use unwrap_or_else_error_handle::handle_error;
```

Suppose you have a function that returns a `Result`:

```rust
fn might_fail(success: bool) -> Result<&'static str, &'static str> {
    if success {
        Ok("All good!")
    } else {
        Err("Something went wrong")
    }
}
```

You can handle errors like this:

```rust
let value = might_fail(false)
    .unwrap_or_else(handle_error("An error occurred"));
```

If an error occurs, this will print:

```
An error occurred: Something went wrong
```

and exit the program with status code 1.

## Example

```rust
fn main() {
    let result = Err("file not found");
    let _ = result.unwrap_or_else(handle_error("Failed to open file"));
}
```
