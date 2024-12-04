# A derive implementation for `#[derive(Demo)]`

Modified from [derive-new](https://github.com/nrc/derive-new) by changing `new` to `demo` such as
- `#derive(new)` to `#derive(Demo)`
- `#[new(value = "42")]` to `#[Demo(value = "42")]`
- `Foo::new("Hello");` to `Foo::demo("Hello");`

A `derive(Demo)` attribute creates a `demo` constructor function for the annotated
type. That function takes an argument for each field in the type giving a
trivial constructor. This is useful since as your type evolves you can make the
constructor non-trivial (and add or remove fields) without changing client code
(i.e., without breaking backwards compatibility). It is also the most succinct
way to initialise a struct or an enum.

Implementation uses macros 1.1 custom derive (which works in stable Rust from
1.15 onwards).

`#[no_std]` is fully supported if you switch off the default feature `"std"`.

## Examples

Cargo.toml:

```toml
[dependencies]
derive-demo = "0.1"
```

Include the macro:

* Rust Edition 2015

  ```rust
  #[macro_use]
  extern crate derive_demo;
  ```

* Rust Edition 2018
  ```rust
  use derive_demo::Demo;
  ```

Generating constructor for a simple struct:

```rust
#[derive(Demo)]
struct Bar {
    a: i32,
    b: String,
}

let _ = Bar::demo(42, "Hello".to_owned());
```

Default values can be specified either via `#[Demo(default)]` attribute which removes
the argument from the constructor and populates the field with `Default::default()`,
or via `#[Demo(value = "..")]` which initializes the field with a given expression:

```rust
#[derive(Demo)]
struct Foo {
    x: bool,
    #[Demo(value = "42")]
    y: i32,
    #[Demo(default)]
    z: Vec<String>,
}

let _ = Foo::demo(true);
```

To make type conversion easier, `#[Demo(into)]` attribute changes the parameter type
to `impl Into<T>`, and populates the field with `value.into()`:

```rust
#[derive(Demo)]
struct Foo {
    #[Demo(into)]
    x: String,
}

let _ = Foo::demo("Hello");
```

For iterators/collections, `#[Demo(into_iter = "T")]` attribute changes the parameter type
to `impl IntoIterator<Item = T>`, and populates the field with `value.into_iter().collect()`:

```rust
#[derive(Demo)]
struct Foo {
    #[Demo(into_iter = "bool")]
    x: Vec<bool>,
}

let _ = Foo::demo([true, false]);
let _ = Foo::demo(Some(true));
```

Generic types are supported; in particular, `PhantomData<T>` fields will be not
included in the argument list and will be initialized automatically:

```rust
use std::marker::PhantomData;

#[derive(new)]
struct Generic<'a, T: Default, P> {
    x: &'a str,
    y: PhantomData<P>,
    #[Demo(default)]
    z: T,
}

let _ = Generic::<i32, u8>::demo("Hello");
```

For enums, one constructor method is generated for each variant, with the type
name being converted to snake case; otherwise, all features supported for
structs work for enum variants as well:

```rust
#[derive(Demo)]
enum Enum {
    FirstVariant,
    SecondVariant(bool, #[Demo(default)] u8),
    ThirdVariant { x: i32, #[Demo(value = "vec![1]")] y: Vec<u8> }
}

let _ = Enum::demo_first_variant();
let _ = Enum::demo_second_variant(true);
let _ = Enum::demo_third_variant(42);
```
