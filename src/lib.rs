#![allow(incomplete_features)]
#![feature(specialization)]

mod bit_util {
    static BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    #[inline]
    pub unsafe fn get_bit_raw(data: *const u8, i: usize) -> bool {
        (*data.add(i >> 3) & BIT_MASK[i & 7]) != 0
    }
}

trait ArrowPrimitiveType {
    type Native: Copy;
}

trait ArrowNumericType: ArrowPrimitiveType {}

struct BooleanType {}
impl ArrowPrimitiveType for BooleanType {
    type Native = bool;
}

struct PrimitiveArray<T: ArrowPrimitiveType> {
    raw_values: *const T::Native,   // actually wrapped in `RawPtrBox`
}

trait PrimitiveArrayOps<T: ArrowPrimitiveType> {
    fn value(&self, i: usize) -> T::Native;
}

impl<T: ArrowPrimitiveType> PrimitiveArrayOps<T> for PrimitiveArray<T> {
    // This impl must exist so that `PrimitiveArray::<T: ArrowPrimitiveType>::value(i)` can be
    // called without knowing the type of `T`.
    
    default fn value(&self, _: usize) -> T::Native {
        unimplemented!()
    }
}

impl<T: ArrowNumericType> PrimitiveArrayOps<T> for PrimitiveArray<T> {
    fn value(&self, i: usize) -> T::Native {
        unsafe { *(self.raw_values.add(i)) }
    }
}

impl PrimitiveArrayOps<BooleanType> for PrimitiveArray<BooleanType> {
    fn value(&self, i: usize) -> bool {
        unsafe { bit_util::get_bit_raw(self.raw_values as *const u8, i) }
    }
}
