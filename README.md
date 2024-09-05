# intfinity

[`intfinity`](https://github.com/samarthsoup/intfinity/blob/master/src/intfinity.rs) is a Rust crate that provides a generic type for representing and working with both finite and infinite numeric values. It supports operations that automatically handle overflow and underflow, mapping them to positive or negative infinity as appropriate.

## Features

- **Generic Infinity Representation**: Define numeric types that can represent positive and negative infinity alongside finite values.
- **Checked Arithmetic**: Implementations of `Add`, `Sub`, `Mul`, and `Div` traits with built-in overflow and underflow detection.
- **Custom Traits**: Traits like [`Zero`](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L1), [`Negate`](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L8), [Unsigned](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L30),[`CheckedAdd`](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L13), [`CheckedSub`](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L17), [`CheckedMul`](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L21), and [`CheckedDiv`](https://github.com/samarthsoup/intfinity/blob/27a937e3e4aea0197783b634083f06d0a3fae59f/src/traits.rs#L26) allow for extensibility to custom numeric types.

## Getting Started

Add `Intfinity` to your `Cargo.toml`:

```toml
[dependencies]
intfinity = "0.3.1"
```

## Example Usage
```
use intfinity::DoubleInfiniteNumber;

fn main() {
    let a = DoubleInfiniteNumber::new(42);
    let b = DoubleInfiniteNumber::PosInfinity;
    
    let result = a + b;
    
    println!("result = {}", result); // stdout will be "+infinity"
}
```

## Traits

Intfinity comes with several traits that are essential for working with numeric types:

    Zero: Defines the concept of zero for a type.
    Negate: Provides a method to negate a value.
    CheckedAdd, CheckedSub, CheckedMul, CheckedDiv: Provide arithmetic operations with overflow and underflow checks.

## License
This project is under the MIT license.


