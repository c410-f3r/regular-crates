//! COO

#![no_main]

use rand as _;
use rayon as _;

use libfuzzer_sys::fuzz_target;
use ndstruct::coo::CooVec;

type Array = [usize; 3];

#[derive(Debug, arbitrary::Arbitrary)]
struct Values {
  data: Vec<(Array, i32)>,
  dims: Array,
  value: Array,
}

fuzz_target!(|values: Values| {
  let real_data = values.data.into_iter().collect::<Vec<_>>();

  let coo: CooVec<i32, 3> = if let Ok(r) = CooVec::new(values.dims, real_data) {
    r
  } else {
    return;
  };

  let _ = coo.value(values.value);
});
