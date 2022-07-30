# Result

There is support for Rust's `std::Result`, where you can return either a value or an error.

## Example

```rust,noplayground

pub enum CustomError {
    Error0(String),
    Error1(u32),
}

pub fn return_err_custom_error() -> Result<u32, CustomError> {
    Err(CustomError::Error1(3))
}
```

Becomes something that can be used like this:

```Dart
try {
      final r = await api.returnErrCustomError();
      print("received $r");
    } catch (e) {
      print('dart catch e: $e');
      expect(e, isA<CustomError>());
    }
}
```