#![allow(dead_code)]
#![warn(clippy::source_item_ordering)]

enum EnumOrdered {
    A,
    B,
    C,
}

enum EnumUnordered {
    A,
    C,
    B,
}

struct StructOrdered {
    a: bool,
    b: bool,
    c: bool,
}

struct StructUnordered {
    a: bool,
    c: bool,
    b: bool,
    d: bool,
}

struct StructUnorderedGeneric<T> {
    _1: std::marker::PhantomData<T>,
    a: bool,
    c: bool,
    b: bool,
    d: bool,
}

trait TraitOrdered {
    const A: bool;
    const B: bool;
    const C: bool;

    type SomeType;

    fn a();
    fn b();
    fn c();
}

trait TraitUnordered {
    const A: bool;
    const C: bool;
    const B: bool;

    type SomeType;

    fn a();
    fn c();
    fn b();
}

trait TraitUnorderedItemKinds {
    type SomeType;

    const A: bool;
    const B: bool;
    const C: bool;

    fn a();
    fn b();
    fn c();
}

impl TraitUnordered for StructUnordered {
    const A: bool = false;
    const C: bool = false;
    const B: bool = false;

    type SomeType = ();

    fn a() {}
    fn c() {}
    fn b() {}
}

impl TraitUnorderedItemKinds for StructUnordered {
    type SomeType = ();

    const A: bool = false;
    const B: bool = false;
    const C: bool = false;

    fn a() {}
    fn b() {}
    fn c() {}
}

fn main() {
    // test code goes here
}
