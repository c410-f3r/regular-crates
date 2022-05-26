/// A wrapper around `I: Iterator` to workaround trait implementation conflicts
#[derive(Debug)]
pub struct IterWrapper<I>(
  /// Iterator
  pub I,
)
where
  I: Iterator;

#[cfg(feature = "serde")]
mod serde {
  use crate::IterWrapper;
  use serde::{ser::SerializeSeq, Serialize, Serializer};

  impl<I> Serialize for IterWrapper<I>
  where
    I: Clone + Iterator,
    I::Item: Serialize,
  {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut seq = serializer.serialize_seq(None)?;
      for elem in self.0.clone() {
        seq.serialize_element(&elem)?;
      }
      seq.end()
    }
  }
}
