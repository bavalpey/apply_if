<!--
SPDX-FileCopyrightText: 2024 ApplyIf Contributors

SPDX-License-Identifier: CC-BY-SA-4.0
-->


# ApplyIf: Seamless Conditionals for Builder Chains

[![CI](https://github.com/bavalpey/apply_if/actions/workflows/tests.yml/badge.svg)](https://github.com/bavalpey/apply_if/actions/workflows/tests.yml) [![REUSE status](https://api.reuse.software/badge/github.com/bavalpey/apply_if)](https://api.reuse.software/info/github.com/bavalpey/apply_if)

This crate provides the functions `apply_if` and `apply_if_some` for consuming
builders, as well as `apply_if_mut` and `apply_if_some_mut` for mutable builder
chains. They allow us to conditionally invoke builder functions while keeping
the nice chaining syntax.

## Examples

These examples use the [`derive_builder`](https://crates.io/crates/derive_builder)
crate to create a builder and show how to use the `apply_if...` functions to
conditionally invoke builder functions without breaking the chain.

### Mutable Builders

This example uses a mutable builder pattern, which is why `apply_if_mut`
and `apply_if_some_mut` are used.

```rust
extern crate derive_builder;
use derive_builder::Builder;
use apply_if::ApplyIf;

#[derive(Debug,PartialEq,Builder)]
struct Value{
  #[builder(default = 123)]
  first: i32,
  #[builder(default = 42.)]
  second: f32,
}

fn main () {
    // apply_if_mut only applies the function
    // if the condition is true
    let value = ValueBuilder::default()
        .apply_if_mut(true, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 100, second: 1.337});

    let value = ValueBuilder::default()
        .apply_if_mut(false, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 123, second: 1.337});

    // apply_if_some_mut only applies the function
    // if the optional contains a value
    
    let value = ValueBuilder::default()
        .apply_if_some_mut(Some(100), |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 100, second: 1.337});

    let value = ValueBuilder::default()
        .apply_if_some_mut(None, |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 123, second: 1.337});
}
```

### Immutable Builders

This example uses a consuming builder pattern, which is why `apply_if`
and `apply_if_some` are used.

```rust
extern crate derive_builder;
use derive_builder::Builder;
use apply_if::ApplyIf;

#[derive(Debug,PartialEq,Builder)]
#[builder(pattern = "immutable")]
struct Value{
  #[builder(default = 123)]
  first: i32,
  #[builder(default = 42.)]
  second: f32,
}

fn main () {
    // apply_if only applies the function
    // if the condition is true
    let value = ValueBuilder::default()
        .apply_if(true, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 100, second: 1.337});

    let value = ValueBuilder::default()
        .apply_if(false, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 123, second: 1.337});

    // apply_if_some only applies the function
    // if the optional contains a value
    let value = ValueBuilder::default()
        .apply_if_some(Some(100), |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 100, second: 1.337});

    let value = ValueBuilder::default()
        .apply_if_some(None, |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value, Value{first: 123, second: 1.337});
}
```


## License

The code in this repository is licensed under either the [MIT License][mit] or
the [Apache License 2.0][apache], at your option. Documentation, specifically
the documentation comments and all markdown files, are licensed under the
[Creative Commons ShareAlike 4.0 license][cc-by-sa]. Generated files are
provided under [CC0-1.0][cc0].

[mit]: LICENSES/MIT.txt
[apache]: LICENSES/Apache-2.0.txt
[cc-by-sa]: LICENSES/CC-BY-SA-4.0.txt
[cc0]: LICENSES/CC0-1.0.txt