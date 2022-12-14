use core::{
  borrow::{Borrow, BorrowMut},
  ops::{Deref, DerefMut},
  slice::{Iter, IterMut},
};

/// Used for serialization, de-serialization or to construct custom arrays.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArrayWrapper<T, const N: usize>(
  /// The actual array
  pub [T; N],
);

impl<T, const N: usize> ArrayWrapper<T, N> {
  /// Creates an array `[T; N]` where each array element `T` is returned by the `cb` call.
  #[inline]
  pub fn from_fn(mut cb: impl FnMut(usize) -> T) -> Self {
    let mut idx = 0;
    Self([(); N].map(|_| {
      let res = cb(idx);
      idx = idx.wrapping_add(1);
      res
    }))
  }

  /// Creates an array `ArrayWrapper` where each fallible array element `T` is returned by the `cb` call.
  /// Unlike [`ArrayWrapper::from_fn`], where the element creation can't fail, this version will return an error
  /// if any element creation was unsuccessful.
  #[inline]
  pub fn try_from_fn<E>(cb: impl FnMut(usize) -> Result<T, E>) -> Result<Self, E> {
    Ok(Self(here_be_dragons::try_from_fn(cb)?))
  }
}

impl<T, const N: usize> AsRef<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn as_ref(&self) -> &[T; N] {
    self
  }
}

impl<T, const N: usize> AsMut<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn as_mut(&mut self) -> &mut [T; N] {
    self
  }
}

impl<T, const N: usize> Borrow<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn borrow(&self) -> &[T; N] {
    self
  }
}

impl<T, const N: usize> BorrowMut<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [T; N] {
    self
  }
}

impl<T, const N: usize> Default for ArrayWrapper<T, N>
where
  T: Default,
{
  #[inline]
  fn default() -> Self {
    Self::from_fn(|_| T::default())
  }
}

impl<T, const N: usize> Deref for ArrayWrapper<T, N> {
  type Target = [T; N];

  #[inline]
  fn deref(&self) -> &[T; N] {
    &self.0
  }
}

impl<T, const N: usize> DerefMut for ArrayWrapper<T, N> {
  #[inline]
  fn deref_mut(&mut self) -> &mut [T; N] {
    &mut self.0
  }
}

impl<T, const N: usize> From<[T; N]> for ArrayWrapper<T, N> {
  #[inline]
  fn from(from: [T; N]) -> Self {
    Self(from)
  }
}

impl<'array, T, const N: usize> IntoIterator for &'array ArrayWrapper<T, N> {
  type IntoIter = Iter<'array, T>;
  type Item = &'array T;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'array, T, const N: usize> IntoIterator for &'array mut ArrayWrapper<T, N> {
  type IntoIter = IterMut<'array, T>;
  type Item = &'array mut T;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

// Code was copied from Rustc, therefore, no UB should be triggered (at least theoretically)
mod here_be_dragons {
  #![allow(clippy::as_conversions, clippy::mem_forget, trivial_casts, unsafe_code)]

  use core::{
    mem::{self, MaybeUninit},
    ptr::{self, addr_of, addr_of_mut},
  };

  #[inline]
  pub(super) fn try_from_fn<E, T, const N: usize>(
    cb: impl FnMut(usize) -> Result<T, E>,
  ) -> Result<[T; N], E> {
    let mut iter = (0..N).map(cb);
    debug_assert!(N <= iter.size_hint().1.unwrap_or(usize::MAX));
    debug_assert!(N <= iter.size_hint().0);
    // SAFETY: covered by the function contract.
    unsafe { try_collect_into_array(&mut iter).unwrap_unchecked() }
  }

  #[allow(
    // Takes ownership to prevent a double reference.
    clippy::needless_pass_by_value
  )]
  unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    unsafe { (addr_of!(array).cast::<[T; N]>()).read() }
  }

  fn try_collect_into_array<E, T, const N: usize>(
    iter: &mut impl Iterator<Item = Result<T, E>>,
  ) -> Option<Result<[T; N], E>> {
    struct Guard<'array, T, const N: usize> {
      array_mut: &'array mut [MaybeUninit<T>; N],
      initialized: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
      fn drop(&mut self) {
        debug_assert!(self.initialized <= N);

        // SAFETY: this slice will contain only initialized objects.
        unsafe {
          ptr::drop_in_place(slice_assume_init_mut(
            self.array_mut.get_unchecked_mut(..self.initialized),
          ));
        }
      }
    }

    if N == 0 {
      // SAFETY: An empty array is always inhabited and has no validity invariants.
      return unsafe { Some(mem::zeroed()) };
    }

    let mut array = uninit_array::<T, N>();
    let mut guard = Guard { array_mut: &mut array, initialized: 0 };

    for item_rslt in iter {
      let item = match item_rslt {
        Err(err) => {
          return Some(Err(err));
        }
        Ok(elem) => elem,
      };

      // SAFETY: `guard.initialized` starts at 0, is increased by one in the
      // loop and the loop is aborted once it reaches N (which is
      // `array.len()`).
      unsafe {
        let _ = guard.array_mut.get_unchecked_mut(guard.initialized).write(item);
      }
      guard.initialized = guard.initialized.wrapping_add(1);

      // Check if the whole array was initialized.
      if guard.initialized == N {
        mem::forget(guard);

        // SAFETY: the condition above asserts that all elements are
        // initialized.
        let out = unsafe { array_assume_init(array) };
        return Some(Ok(out));
      }
    }

    None
  }

  unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(addr_of_mut!(*slice) as *mut [T]) }
  }

  const fn uninit_array<T, const LEN: usize>() -> [MaybeUninit<T>; LEN] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    unsafe { MaybeUninit::<[MaybeUninit<T>; LEN]>::uninit().assume_init() }
  }
}

#[cfg(feature = "serde")]
mod serde {
  use crate::{ArrayWrapper, ArrayWrapperRef};
  use core::{fmt::Formatter, marker::PhantomData};
  use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
  };

  impl<'de, T, const N: usize> Deserialize<'de> for ArrayWrapper<T, N>
  where
    T: Deserialize<'de>,
  {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

      impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
      where
        T: Deserialize<'de>,
      {
        type Value = ArrayWrapper<T, N>;

        #[inline]
        fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
          formatter.write_fmt(format_args!("an array with {N} elements"))
        }

        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
          A: SeqAccess<'de>,
        {
          ArrayWrapper::try_from_fn(|_| {
            seq.next_element::<T>()?.ok_or_else(|| {
              de::Error::invalid_length(N, &"Array need more data to be constructed")
            })
          })
        }
      }

      deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
    }
  }

  impl<T, const N: usize> Serialize for ArrayWrapper<T, N>
  where
    T: Serialize,
  {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      ArrayWrapperRef::from(&self.0).serialize(serializer)
    }
  }
}
