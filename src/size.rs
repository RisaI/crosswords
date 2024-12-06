use ahash::HashMap;
use fxhash::FxHashMap;
use smallvec::SmallVec;

pub trait EstimateSize {
    fn estimate_size(&self) -> usize;
}

impl<T: EstimateSize> EstimateSize for Vec<T> {
    fn estimate_size(&self) -> usize {
        size_of::<Vec<T>>() + self.iter().map(EstimateSize::estimate_size).sum::<usize>()
    }
}

impl<T: EstimateSize> EstimateSize for Box<[T]> {
    fn estimate_size(&self) -> usize {
        size_of::<Box<[T]>>() + self.iter().map(EstimateSize::estimate_size).sum::<usize>()
    }
}

impl<T: EstimateSize, const N: usize> EstimateSize for [T; N] {
    fn estimate_size(&self) -> usize {
        self.iter().map(EstimateSize::estimate_size).sum::<usize>()
    }
}

impl<T: EstimateSize, const N: usize> EstimateSize for SmallVec<[T; N]> {
    fn estimate_size(&self) -> usize {
        if self.len() > self.inline_size() {
            size_of::<SmallVec<[T; N]>>()
                + self.iter().map(EstimateSize::estimate_size).sum::<usize>()
        } else {
            size_of::<SmallVec<[T; N]>>()
        }
    }
}

impl<K: EstimateSize, V: EstimateSize> EstimateSize for HashMap<K, V> {
    fn estimate_size(&self) -> usize {
        size_of::<HashMap<K, V>>()
            + self
                .iter()
                .map(|(k, v)| k.estimate_size() + v.estimate_size())
                .sum::<usize>()
            + (self.capacity() - self.len()) * (size_of::<K>() + size_of::<V>())
    }
}

impl<K: EstimateSize, V: EstimateSize> EstimateSize for FxHashMap<K, V> {
    fn estimate_size(&self) -> usize {
        size_of::<HashMap<K, V>>()
            + self
                .iter()
                .map(|(k, v)| k.estimate_size() + v.estimate_size())
                .sum::<usize>()
            + (self.capacity() - self.len()) * (size_of::<K>() + size_of::<V>())
    }
}

macro_rules! impl_estimate_size {
    ( $( $x:ty ),* ) => {
        $(
            impl EstimateSize for $x {
                fn estimate_size(&self) -> usize {
                    size_of::<$x>()
                }
            }
        )*
    };
}

impl_estimate_size!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_estimate_size!(crate::Direction);

impl<A, B> EstimateSize for (A, B)
where
    A: EstimateSize,
    B: EstimateSize,
{
    fn estimate_size(&self) -> usize {
        self.0.estimate_size() + self.1.estimate_size()
    }
}

impl<A, B, C> EstimateSize for (A, B, C)
where
    A: EstimateSize,
    B: EstimateSize,
    C: EstimateSize,
{
    fn estimate_size(&self) -> usize {
        self.0.estimate_size() + self.1.estimate_size() + self.2.estimate_size()
    }
}
