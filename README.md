# DistCPU

**WARNING**: This library is nowhere near a stable version. Do not use this crate yet.

DistCPU is a distributable symbolic computation library written in Rust. Thanks to Rust's type system, DistCPU can perform calculations on values that are not yet known. This allows for distributed computation for tasks that would otherwise be impossible to parallelize, like conditional branching.

## Example

### Basic

```rust
use distcpu::prelude::*;

let a = 1;
let b = 2;

let c = Add::new(Mul::new(b, 5), a);
let c = c.resolve().await;

assert_eq!(c, 11);
```

### With operators

```rust
use hypercpu::prelude::*;

let a = Value::new(1);
let b = a * 2;
let c = b + 3;

assert_eq!(c.resolve().await, 5);
```

## Stability

DistCPU is still in very early development. I make no guarantees that this will ever be finished, but I'm happy to work on this with students from [Lernib](https://github.com/lernib)

## License

DistCPU is licensed under the [MIT License](./LICENSE-MIT) OR [Apache License 2.0](./LICENSE-APACHE).