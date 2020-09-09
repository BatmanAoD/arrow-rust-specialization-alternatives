mod bit_util {
    static BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    #[inline]
    pub unsafe fn get_bit_raw(data: *const u8, i: usize) -> bool {
        (*data.add(i >> 3) & BIT_MASK[i & 7]) != 0
    }
}

trait ArrowPrimitiveType {
    type Native: Copy;
    fn index(raw_ptr: *const Self::Native, i: usize) -> Self::Native;
}

trait ArrowNumericType: ArrowPrimitiveType {
    fn index(raw_ptr: *const Self::Native, i: usize) -> Self::Native {
        unsafe { *(raw_ptr.add(i)) }
    }
}

struct BooleanType {}
impl ArrowPrimitiveType for BooleanType {
    type Native = bool;
    fn index(raw_ptr: *const bool, i: usize) -> bool {
        unsafe { bit_util::get_bit_raw(raw_ptr as *const u8, i) }
    }
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
    
    fn value(&self, i: usize) -> T::Native {
        T::index(self.raw_values, i)
    }
}
