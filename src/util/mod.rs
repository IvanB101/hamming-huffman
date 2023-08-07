pub mod bitarr;
pub mod string;
pub mod typed_io;

pub trait CeilDiv {
    fn ceil_div(&self, divisor: Self) -> Self;
}

macro_rules! ceil_div {
    ($type:ty) => {
        impl CeilDiv for $type {
            fn ceil_div(&self, divisor: Self) -> Self {
                if self % divisor != 0 {
                    self / divisor + 1
                } else {
                    self / divisor
                }
            }
        }
    };
}

ceil_div!(u8);
ceil_div!(u16);
ceil_div!(u32);
ceil_div!(u64);
ceil_div!(i8);
ceil_div!(i16);
ceil_div!(i32);
ceil_div!(i64);
