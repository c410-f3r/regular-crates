use crate::{Clear, Push, Truncate, WithCapacity};
use core::fmt::Write;
use core::ops::{Deref, DerefMut};

/// Dynamic Contiguous Collection
///
/// A growable vector-like abstraction for generic elements.
pub trait DynContigColl<T>:
  AsRef<T>
  + Clear
  + Default
  + Deref<Target = T>
  + DerefMut
  + crate::Extend<T>
  + Push<T, Error = crate::Error, Output = ()>
  + Truncate<Input = usize, Output = ()>
  + WithCapacity<Input = usize>
  + Write
{
}

impl<T, U> DynContigColl<T> for U where
  U: AsRef<T>
    + Clear
    + Default
    + Deref<Target = T>
    + DerefMut
    + crate::Extend<T>
    + Push<T, Error = crate::Error, Output = ()>
    + Truncate<Input = usize, Output = ()>
    + WithCapacity<Input = usize>
    + Write
{
}
