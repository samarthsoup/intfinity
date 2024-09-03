# Intfinity

`Intfinity` is a Rust crate that provides a generic type for representing and working with both finite and infinite numeric values. It supports operations that automatically handle overflow and underflow, mapping them to positive or negative infinity as appropriate.

## Features

- **Generic Infinity Representation**: Define numeric types that can represent positive and negative infinity alongside finite values.
- **Checked Arithmetic**: Implementations of `Add`, `Sub`, `Mul`, and `Div` traits with built-in overflow and underflow detection.
- **Custom Traits**: Traits like `Zero`, `Negate`, `CheckedAdd`, `CheckedSub`, `CheckedMul`, and `CheckedDiv` allow for extensibility to custom numeric types.

## Getting Started

Add `Intfinity` to your `Cargo.toml`:

```toml
[dependencies]
intfinity = "0.1.0"
```

## Example Usage
```
use intfinity::Intfinity;

fn main() {
    let a = Intfinity::new(42);
    let b = Intfinity::PosInfinity;
    
    let result = a + b;
    
    match result {
        Intfinity::Finite(value) => println!("Finite: {}", value),
        Intfinity::PosInfinity => println!("Positive Infinity"),
        Intfinity::NegInfinity => println!("Negative Infinity"),
    }
}
```

## Traits

Intfinity comes with several traits that are essential for working with numeric types:

    Zero: Defines the concept of zero for a type.
    Negate: Provides a method to negate a value.
    CheckedAdd, CheckedSub, CheckedMul, CheckedDiv: Provide arithmetic operations with overflow and underflow checks.

## License
This project is under the MIT license.


