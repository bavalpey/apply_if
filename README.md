# ApplyIf: Seamless Conditionals for Builder Chains

[![CI](https://github.com/bavalpey/apply_if/actions/workflows/tests.yml/badge.svg)](https://github.com/bavalpey/apply_if/actions/workflows/tests.yml)

This crates provides the functions `apply_if` and `apply_if_some` for consuming
builders, as well as `apply_if_mut` and `apply_if_some_mut` for mutable builder
chains. They allow us to conditionally invoke builder functions while keeping
the nice chaining syntax.

# Examples

This example uses the [`derive_builder`](https://crates.io/crates/derive_builder)
crate to derive a builder and then uses the `apply_if_...` functions to show
how to conditionally invoke builder functions without breaking the chain.

## Mutable Builders

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
    let value1 = ValueBuilder::default()
        .apply_if_mut(true, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value1, Value{first: 100, second: 1.337});

    let value2 = ValueBuilder::default()
        .apply_if_mut(false, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value2, Value{first: 123, second: 1.337});

    // apply_if_some_mut only applies the function
    // if the optional contains a value
    
    let value3 = ValueBuilder::default()
        .apply_if_some_mut(Some(100), |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value3, Value{first: 100, second: 1.337});

    let value3 = ValueBuilder::default()
        .apply_if_some_mut(None, |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value3, Value{first: 123, second: 1.337});
}
```

# Immutable Builders

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
    // apply_if_mut only applies the function
    // if the condition is true
    let value1 = ValueBuilder::default()
        .apply_if(true, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value1, Value{first: 100, second: 1.337});

    let value2 = ValueBuilder::default()
        .apply_if(false, |builder| builder.first(100))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value2, Value{first: 123, second: 1.337});

    // apply_if_some_mut only applies the function
    // if the optional contains a value
    
    let value3 = ValueBuilder::default()
        .apply_if_some(Some(100), |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value3, Value{first: 100, second: 1.337});

    let value3 = ValueBuilder::default()
        .apply_if_some(None, |builder,val| builder.first(val))
        .second(1.337)
        .build()
        .unwrap();
    assert_eq!(value3, Value{first: 123, second: 1.337});
}
```
