# HyperCPU

**WARNING**: This library is nowhere near a stable version. Do not use this crate yet.

HyperCPU is a distributable symbolic computation library written in Rust. Thanks to Rust's type system, HyperCPU can perform calculations on values that are not yet known. This allows for distributed computation for tasks that would otherwise be impossible to parallelize, like conditional branching.

## Example

```rust
use hypercpu::prelude::*;

let a = 1;
let b = 2;

let c = Add::new(Mul::new(b, 5), a);
let c = c.resolve().await;

assert_eq!(c, 11);
```

## Stability

HyperCPU is still in very early development. It will be a component of the [Circe Project](https://github.com/carlosskii/circe), which is also in early development.

## License

HyperCPU is licensed under the [MIT License](./LICENSE-MIT) OR [Apache License 2.0](./LICENSE-APACHE).