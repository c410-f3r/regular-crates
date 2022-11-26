use crate::{Capacity, Clear, Push, Truncate, WithCapacity};
use core::ops::{Deref, DerefMut};

/// Dynamic Contiguous Collection
///
/// A growable vector-like abstraction for generic elements.
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
/// fn stuff<T>(dcc: &mut T)
/// where
///     T: cl_aux::DynContigColl<u8>
/// {
///     dcc.clear();
///     dcc.extend([4, 5, 6]).unwrap();
///     dcc.truncate(1);
/// }
///
/// let mut dcc = vec![0, 1, 2, 3];
/// stuff(&mut dcc);
/// assert_eq!(dcc, &[4]);
/// ```
pub trait DynContigColl<T>:
  AsRef<[T]>
  + Clear
  + Capacity
  + Default
  + Deref<Target = [T]>
  + DerefMut
  + crate::Extend<T, Error = crate::Error, Output = ()>
  + Push<T, Error = crate::Error, Output = ()>
  + Truncate<Input = usize, Output = ()>
  + WithCapacity<Input = usize>
{
}

impl<T, U> DynContigColl<T> for U where
  U: AsRef<[T]>
    + Clear
    + Capacity
    + Default
    + Deref<Target = [T]>
    + DerefMut
    + crate::Extend<T, Error = crate::Error, Output = ()>
    + Push<T, Error = crate::Error, Output = ()>
    + Truncate<Input = usize, Output = ()>
    + WithCapacity<Input = usize>
{
}
