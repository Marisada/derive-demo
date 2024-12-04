#![deny(non_snake_case)]

#[macro_use]
extern crate derive_demo;

use std::fmt::Debug;

/// A struct with no fields.
#[derive(Demo, PartialEq, Debug)]
pub struct Foo {}

#[test]
fn test_empty_struct() {
    let x = Foo::demo();
    assert_eq!(x, Foo {});
}

/// A unit struct.
#[derive(Demo, PartialEq, Debug)]
pub struct Baz;

#[test]
fn test_unit_struct() {
    let x = Baz::demo();
    assert_eq!(x, Baz);
}

/// A struct with fields.
#[derive(Demo, PartialEq, Debug)]
#[Demo(visibility = "pub(crate)")]
pub struct Bar {
    pub x: i32,
    pub y: String,
}

#[test]
fn test_simple_struct() {
    let x = Bar::demo(42, "Hello".to_owned());
    assert_eq!(
        x,
        Bar {
            x: 42,
            y: "Hello".to_owned()
        }
    );
}

/// A struct with a lifetime parameter.
#[derive(Demo, PartialEq, Debug)]
pub struct Intersection<'scene> {
    pub object: &'scene Bar,
    pub normal: Foo,
    pub point: Foo,
    pub t: f64,
}

#[test]
fn test_struct_with_lifetime() {
    let b = Bar::demo(42, "Hello".to_owned());
    let x = Intersection::demo(&b, Foo::demo(), Foo::demo(), 42.0);
    assert_eq!(
        x,
        Intersection {
            object: &b,
            normal: Foo {},
            point: Foo {},
            t: 42.0
        }
    );
}

/// A struct with generics and bounds.
#[derive(Demo, PartialEq, Debug)]
pub struct Qux<T: Debug + PartialEq, U: Debug + PartialEq> {
    pub f1: T,
    pub f2: Vec<U>,
    pub f3: i32,
}

#[test]
fn test_struct_with_bounds() {
    let x = Qux::demo("Hello!", Vec::<String>::new(), 42);
    assert_eq!(
        x,
        Qux {
            f1: "Hello!",
            f2: vec![],
            f3: 42
        }
    );

    let x: Qux<&'static str, String> = Qux::demo("Hello!", Vec::<String>::new(), 42);
    assert_eq!(
        x,
        Qux {
            f1: "Hello!",
            f2: vec![],
            f3: 42
        }
    );

    let x = Qux::<_, String>::demo("Hello!", vec![], 42);
    assert_eq!(
        x,
        Qux {
            f1: "Hello!",
            f2: vec![],
            f3: 42
        }
    );
}

/// A struct with a lifetime parameter, generics and bounds.
#[derive(Demo, PartialEq, Debug)]
pub struct FooBar<'a, T, U>
where
    T: 'a + PartialEq + Debug,
    U: Sized + Send + 'a + PartialEq + Debug,
{
    pub f1: Box<T>,
    pub f2: Vec<&'a U>,
    pub f3: i32,
}

#[test]
fn test_struct_lifetime_bounds() {
    let a = 42;
    let x = FooBar::demo(Box::new("Hello".to_owned()), vec![&a], 42);
    assert_eq!(
        x,
        FooBar {
            f1: Box::new("Hello".to_owned()),
            f2: vec![&a],
            f3: 42
        }
    );
}

/// A tuple struct.
#[derive(Demo, PartialEq, Debug)]
pub struct Tuple(pub i32, pub i32);

#[test]
fn test_simple_tuple_struct() {
    let x = Tuple::demo(5, 6);
    assert_eq!(x, Tuple(5, 6));
}

/// A tuple struct with a lifetime parameter.
#[derive(Demo, PartialEq, Debug)]
pub struct TupleWithLifetime<'a>(pub &'a str);

#[test]
fn test_tuple_struct_lifetime() {
    let x = TupleWithLifetime::demo("Hello");
    assert_eq!(x, TupleWithLifetime("Hello"));
}

#[cfg(feature = "std")]
#[test]
fn test_struct_with_defaults() {
    use std::default::Default;

    /// A struct where fields have default values.
    #[derive(Demo, PartialEq, Debug)]
    pub struct Waldo<T: PartialEq + Debug + Default> {
        #[Demo(default)]
        pub x: i32,
        pub y: u8,
        #[Demo(default)]
        pub z: T,
    }

    let x = Waldo::<Vec<String>>::demo(42);
    assert_eq!(
        x,
        Waldo {
            x: 0,
            y: 42,
            z: vec![]
        }
    );
}

#[cfg(feature = "std")]
#[test]
fn test_struct_with_into() {
    #[derive(Demo, PartialEq, Debug)]
    pub struct Foo {
        #[Demo(into)]
        pub value: String,
    }

    assert_eq!(
        Foo::demo("bar"),
        Foo {
            value: "bar".to_string(),
        }
    );
}

#[cfg(feature = "std")]
#[test]
fn test_struct_with_into_iter() {
    #[derive(Demo, PartialEq, Debug)]
    pub struct Foo {
        #[Demo(into_iter = "bool")]
        pub values: Vec<bool>,
    }

    assert_eq!(
        Foo::demo([true, false, true]),
        Foo {
            values: vec![true, false, true]
        }
    );

    assert_eq!(
        Foo::demo(Some(false)),
        Foo {
            values: vec![false]
        }
    );

    assert_eq!(Foo::demo(None), Foo { values: vec![] });
}

/// A struct where fields have explicitly provided defaults.
#[derive(Demo, PartialEq, Debug)]
pub struct Fred {
    #[Demo(value = "1 + 2")]
    pub x: i32,
    pub y: String,
    #[Demo(value = "vec![-42, 42]")]
    pub z: Vec<i8>,
}

#[test]
fn test_struct_with_values() {
    let x = Fred::demo("Fred".to_owned());
    assert_eq!(
        x,
        Fred {
            x: 3,
            y: "Fred".to_owned(),
            z: vec![-42, 42]
        }
    );
}

#[cfg(feature = "std")]
#[test]
fn test_struct_mixed_defaults() {
    /// A struct with defaults and specified values.
    #[derive(Demo, PartialEq, Debug)]
    pub struct Thud {
        #[Demo(value = r#""Thud".to_owned()"#)]
        pub x: String,
        #[Demo(default)]
        pub y: String,
    }

    let x = Thud::demo();
    assert_eq!(
        x,
        Thud {
            x: "Thud".to_owned(),
            y: String::new()
        }
    );
}

#[cfg(feature = "std")]
#[test]
fn test_struct_phantom_data() {
    use std::marker::PhantomData;

    /// A generic struct with PhantomData member.
    #[derive(Demo, PartialEq, Debug)]
    pub struct Bob<T: PartialEq + Debug> {
        pub a: i32,
        pub b: PhantomData<T>,
    }
    let x = Bob::<i32>::demo(42);
    assert_eq!(
        x,
        Bob {
            a: 42,
            b: PhantomData
        }
    );
}

#[cfg(feature = "std")]
#[test]
fn test_tuple_with_defaults() {
    use std::default::Default;

    /// A tuple struct where fields have default values.
    #[derive(Demo, PartialEq, Debug)]
    pub struct Boom<T: PartialEq + Debug + Default>(
        #[Demo(default)] pub i32,
        pub u8,
        #[Demo(default)] pub T,
    );

    let x = Boom::<Vec<String>>::demo(42);
    assert_eq!(x, Boom(0, 42, vec![]));
}

/// A tuple struct where fields have explicitly provided defaults.
#[derive(Demo, PartialEq, Debug)]
pub struct Moog(
    #[Demo(value = "1 + 2")] pub i32,
    pub String,
    #[Demo(value = "vec![-42, 42]")] pub Vec<i8>,
);

#[test]
fn test_tuple_with_values() {
    let x = Moog::demo("Fred".to_owned());
    assert_eq!(x, Moog(3, "Fred".to_owned(), vec![-42, 42]));
}

#[cfg(feature = "std")]
#[test]
fn test_tuple_mixed_defaults() {
    /// A tuple struct with defaults and specified values.
    #[derive(Demo, PartialEq, Debug)]
    pub struct Crab(
        #[Demo(value = r#""Thud".to_owned()"#)] pub String,
        #[Demo(default)] pub String,
    );

    let x = Crab::demo();
    assert_eq!(x, Crab("Thud".to_owned(), String::new()));
}

#[cfg(feature = "std")]
#[test]
fn test_tuple_phantom_data() {
    use std::marker::PhantomData;

    /// A generic tuple struct with PhantomData member.
    #[derive(Demo, PartialEq, Debug)]
    pub struct Sponge<T: PartialEq + Debug>(pub i32, pub PhantomData<T>);

    let x = Sponge::<i32>::demo(42);
    assert_eq!(x, Sponge(42, PhantomData));
}

/// An enum with unit variants
#[derive(Demo, PartialEq, Debug)]
pub enum Fizz {
    ThisISNotADrill,
    BiteMe,
}

#[test]
fn test_enum_unit_variants() {
    let x = Fizz::demo_this_is_not_a_drill();
    assert_eq!(x, Fizz::ThisISNotADrill);

    let x = Fizz::demo_bite_me();
    assert_eq!(x, Fizz::BiteMe);
}

#[cfg(feature = "std")]
#[test]
fn test_more_involved_enum() {
    use std::default::Default;
    use std::marker::PhantomData;

    /// A more involved enum
    #[derive(Demo, PartialEq, Debug)]
    pub enum Enterprise<T: PartialEq + Debug + Default> {
        Picard,
        Data(
            #[Demo(value = "\"fascinating\".to_owned()")] String,
            #[Demo(default)] T,
        ),
        Spock {
            x: PhantomData<T>,
            y: i32,
        },
    }

    let x = Enterprise::<u8>::demo_picard();
    assert_eq!(x, Enterprise::Picard);

    let x = Enterprise::<u8>::demo_data();
    assert_eq!(x, Enterprise::Data("fascinating".to_owned(), 0u8));

    let x = Enterprise::<u8>::demo_spock(42);
    assert_eq!(
        x,
        Enterprise::Spock {
            x: PhantomData,
            y: 42
        }
    );
}

#[allow(non_snake_case)]
#[derive(Demo, PartialEq, Debug)]
pub struct Upside {
    X: i32,
}

#[cfg_attr(test, allow(non_snake_case))]
#[derive(Demo, PartialEq, Debug)]
pub struct Down {
    X: i32,
}

#[derive(Demo, PartialEq, Debug)]
pub struct All {
    #[allow(missing_docs)]
    pub x: i32,
}
