use crate::{Capacity, CapacityUpperBound, Clear, Length, SizeHint, Truncate};

macro_rules! tuple_impls {
  ($(
    $tuple_len:tt {
      $(($idx:tt) -> $T:ident)*
    }
  )+) => {
    $(
      impl<$( $T, )*> Capacity for ($( $T, )*) {
        #[inline]
        fn capacity(&self) -> usize {
          $tuple_len
        }
      }

      impl<$( $T, )*> CapacityUpperBound for ($( $T, )*) {
        const CAPACITY_UPPER_BOUND: usize = $tuple_len;
      }

      #[allow(non_snake_case)]
      impl<$( $T: Clear, )*> Clear for ($( $T, )*) {
        #[inline]
        fn clear(&mut self) {
          $( self.$idx.clear(); )*
        }
      }

      impl<$( $T, )*> Length for ($( $T, )*) {
        #[inline]
        fn length(&self) -> usize {
          $tuple_len
        }
      }

      impl<$( $T, )*> SizeHint for ($( $T, )*) {
        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
          ($tuple_len, Some($tuple_len))
        }
      }

      #[allow(non_snake_case)]
      impl<$( $T: Truncate, )*> Truncate for ($( $T, )*) {
        type Input = ($( $T::Input, )*);

        #[inline]
        fn truncate(&mut self, input: Self::Input) {
          $( self.$idx.truncate(input.$idx); )*
        }
      }
    )+
  }
}

tuple_impls! {
  1 {
    (0) -> A
  }
  2 {
    (0) -> A
    (1) -> B
  }
  3 {
    (0) -> A
    (1) -> B
    (2) -> C
  }
  4 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
  }
  5 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
  }
  6 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
  }
  7 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
  }
  8 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
  }
  9 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
  }
  10 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
  }
  11 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
    (10) -> K
  }
  12 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
    (10) -> K
    (11) -> L
  }
}
