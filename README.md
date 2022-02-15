# Borrown - Borrowed or owned, simplified for no-std.

[![crates.io](https://img.shields.io/crates/v/borrown?label=latest)](https://crates.io/crates/borrown)
[![Documentation](https://docs.rs/borrown/badge.svg)](https://docs.rs/borrown/)
[![License](https://img.shields.io/crates/l/borrown.svg)]()

Borrow or owned, inspired by [Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html).

Provide common trait implementations over `T`.

## Example

```rust
use borrown::Borrown;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Foo {
    pub _val: usize,
}

let x = Foo { _val: 0 };
let b = Borrown::Borrowed(&x);

let _: &Foo = b.as_ref();
let _: &mut Foo = b.clone().as_mut();
let _: Borrown<'_, Foo> = Default::default();
let _: usize = *b;
let _: bool = b == Borrown::Borrowed(&x);
let _: bool = b <= Borrown::Borrowed(&x);
let _: Borrown<'_, Foo> = b.clone();
let _: Foo = b.into_owned();

println!("{:?}", Borrown::Borrowed(&x));

impl core::ops::Deref for Foo {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self._val
    }
}
```
