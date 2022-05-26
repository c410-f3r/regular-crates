use crate::{
  csl::{CslLineIterMut, CslLineIterRef, CslMut, CslRef},
  ParallelIteratorWrapper, ParallelProducerWrapper,
};
use rayon::iter::{
  plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
  IndexedParallelIterator, ParallelIterator,
};

macro_rules! create_rayon_iter {
  ($csl_rayon_iter:ident, $ref:ident) => {
    impl<'slices, T, const D: usize> ParallelIterator
      for ParallelIteratorWrapper<$csl_rayon_iter<'slices, T, D>>
    where
      T: Send + Sync + 'slices,
    {
      type Item = $ref<'slices, T, D>;

      #[inline]
      fn drive_unindexed<C>(self, consumer: C) -> C::Result
      where
        C: UnindexedConsumer<Self::Item>,
      {
        bridge(self, consumer)
      }

      #[inline]
      fn opt_len(&self) -> Option<usize> {
        Some(self.0.len())
      }
    }

    impl<'slices, T, const D: usize> IndexedParallelIterator
      for ParallelIteratorWrapper<$csl_rayon_iter<'slices, T, D>>
    where
      T: Send + Sync + 'slices,
    {
      #[inline]
      fn drive<C>(self, consumer: C) -> C::Result
      where
        C: Consumer<Self::Item>,
      {
        bridge(self, consumer)
      }

      #[inline]
      fn len(&self) -> usize {
        ExactSizeIterator::len(&self.0)
      }

      #[inline]
      fn with_producer<Cb>(self, callback: Cb) -> Cb::Output
      where
        Cb: ProducerCallback<Self::Item>,
      {
        callback.callback(ParallelProducerWrapper(self.0))
      }
    }

    impl<'slices, T, const D: usize> IntoIterator
      for ParallelProducerWrapper<$csl_rayon_iter<'slices, T, D>>
    where
      T: 'slices,
    {
      type IntoIter = $csl_rayon_iter<'slices, T, D>;
      type Item = <Self::IntoIter as Iterator>::Item;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.0
      }
    }

    impl<'slices, T, const D: usize> Producer
      for ParallelProducerWrapper<$csl_rayon_iter<'slices, T, D>>
    where
      T: Send + Sync + 'slices,
    {
      type IntoIter = $csl_rayon_iter<'slices, T, D>;
      type Item = <Self::IntoIter as Iterator>::Item;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.0
      }

      #[inline]
      fn split_at(self, i: usize) -> (Self, Self) {
        let [a, b] = self.0.split_at(i).unwrap();
        (ParallelProducerWrapper(a), ParallelProducerWrapper(b))
      }
    }
  };
}

create_rayon_iter!(CslLineIterRef, CslRef);
create_rayon_iter!(CslLineIterMut, CslMut);
