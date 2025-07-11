# Bikeshedding

around
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

`serde1` - `serde`

`borsh1` - `borsh`
`borsh1_unstable__schema` - `borsh` schema

`protobuf3` - `protobuf` 
`parity-scale-codec3` - `parity-scale-codec` 
`schemars1` - `schemars` JSON schema 

`proptest1` - `proptest` strategy
`arbitrary1` - `arbitrary`

`arbitrary-int1` - `arbitrary-int` `u2`


## Inspired

- https://docs.rs/sp-arithmetic/latest/sp_arithmetic/enum.ArithmeticError.html