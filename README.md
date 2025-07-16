# bitbound-rs

A Rust crate for safe, efficient array access with boundary protection.

## Overview

`bitbound-rs` provides mechanisms for safe unchecked access and overread operations on arrays through two primary techniques:

- **Bit-bounded access**: Ensures index safety by enforcing array bounds at the bit level. This works by either:
  - Limiting the number of bits in the index
  - Allowing controlled overflow handling

- **Bounded operations**: Provides zero-cost abstractions for array access with compile-time safety guarantees

## Features

- Zero runtime overhead for bounds checking when possible
- Safe array access patterns with configurable behavior
- Support for overread operations with proper boundary handling
- Const assertion utilities for compile-time checks

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitbound = "0.1.0"
```

## License

MIT or Apache-2.0
