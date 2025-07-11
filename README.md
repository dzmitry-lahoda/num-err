# Bikeshedding

```rust
pub enum MathErrors {
    InvalidInput,
    Overflow,
    Underflow,
    DivisionByZero,
}
```

- performance
- only strict string matches, not upper or lowercase coersion for parse and serde
- does not capture backtrace nor errored values
- not float errors nor numerics errors
- JSON serde is string
- unit structs typed variants

## Features

