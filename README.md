## Problem summary using `PrimitiveArrayOps`

`ArrowPrimitiveType` is either `ArrowNumericType` or `BooleanType`.

`PrimitiveArray<T: ArrowPrimitiveType>` is therefore either
`PrimitiveArray<T: ArrowNumericType>` or `PrimitiveArray<BooleanType>`.

The *inherent* `impl` blocks of each type of `PrimitiveArray` include `fn value(i)`. In order to
ensure that `PrimitiveArray<T: ArrowPrimitiveType>` provides `fn value(i)`,
`PrimitiveArrayOps<T>` is provided, which forwards `fn value(i)` to the inherent functions.

The inherent functions necessarily differ depending on whether the element-type is boolean.

The `default` function in the impl of `PrimitiveArrayOps` for `ArrowPrimitiveType` must exist
so that it can be called in generic code using the `ArrowPrimitiveType` bound, but it can't
actually have a sane implementation (it is `unimplemented!()`) because it can't know which
of the two valid bounds (`T: ArrowNumericType` or `T: bool`) is correct.

(Side-question: can the `static_assertions` crate be used to ensure that these
`unimplemented!()` versions are never actually instantiated during monomorphization?)

```rust
trait ArrowPrimitiveType {
    type Native: Copy;
}

trait ArrowNumericType: ArrowPrimitiveType {}

struct BooleanType {}
impl ArrowPrimitiveType for BooleanType {
    type Native = bool;
}

struct<T: ArrowPrimitiveType> PrimitiveArray {
    raw_values: *const T::Native,   // actually wrapped in `RawPtrBox`
}

trait PrimitiveArrayOps<T: ArrowPrimitiveType> {
    fn value(&self, i: usize) -> T::Native;
}

impl<T: ArrowPrimitiveType> PrimitiveArrayOps<T> for PrimitiveArray<T> {
    // This impl must exist so that `PrimitiveArray::<T: ArrowPrimitiveType>::value(i)` can be
    // called without knowing the type of `T`.
    
    fn value(&self, i: usize) -> T::Native {
        // ....?
    }
}
```

## Idea: defer to the `ArrowPrimitiveType` trait somehow?

```rust
trait ArrowPrimitiveType {
    type Native;
    fn index(*const Native, usize) -> Native;
}

impl<T: ArrowPrimitiveType> PrimitiveArrayOps<T> for PrimitiveArray<T> {
    fn value(&self, i: usize) -> T::Native {
        T::index(self.raw_values, i)
    }
}
```

...but this seems to blur separation-of-concerns between datatypes & array (which are separate
modules).

So extend the `ArrowPrimitiveType` trait?

```rust
trait ArrowIndexable {
    fn T::index(*const T::Native, usize) -> T::Native;
}

impl<T: ArrowPrimitiveType> ArrowIndexable for T {
    // ....wait, that's not possible without specialization, is it?
}
```

It almost seems like this just needs a way to tell the compiler "I promise this is
implemented..." ... but that's what trait definitions are for, isn't it?


Can we somehow use default generic types (i.e.
`trait ArrowPrimitiveType<Indexer=DefaultIndexer>`) to improve the situation?
