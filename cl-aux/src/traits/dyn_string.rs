use crate::{Capacity, Clear, Push, Truncate, WithCapacity};
use core::{
  fmt::Write,
  ops::{Deref, DerefMut},
};

/// Dynamic String
///
/// Any owned growing string-like structure that `cl-aux` knows should implement this trait.
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
/// fn stuff<S>(s: &mut S)
/// where
///     S: cl_aux::DynString
/// {
///     s.clear();
///     s.push("World").unwrap();
///     s.truncate(1);
/// }
///
/// let mut s = String::from("Hello");
/// stuff(&mut s);
/// assert_eq!(s, "W");
/// ```
pub trait DynString:
  AsRef<str>
  + Capacity
  + Clear
  + Default
  + Deref<Target = str>
  + DerefMut
  + crate::Extend<char, Error = crate::Error>
  + Push<char, Error = crate::Error, Output = ()>
  + for<'str> Push<&'str str, Error = crate::Error, Output = ()>
  + Truncate<Input = usize, Output = ()>
  + WithCapacity<Input = usize>
  + Write
{
}

impl<T> DynString for T where
  T: AsRef<str>
    + Capacity
    + Clear
    + Default
    + Deref<Target = str>
    + DerefMut
    + crate::Extend<char, Error = crate::Error>
    + Push<char, Error = crate::Error, Output = ()>
    + for<'str> Push<&'str str, Error = crate::Error, Output = ()>
    + Truncate<Input = usize, Output = ()>
    + WithCapacity<Input = usize>
    + Write
{
}
