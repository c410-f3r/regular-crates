//! CSL

#![no_main]

use rand as _;

use libfuzzer_sys::fuzz_target;
use ndstruct::csl::CslVec;
use rayon::prelude::*;

type Array = [usize; 3];

#[derive(Debug, arbitrary::Arbitrary)]
struct Values {
  data: Vec<i32>,
  dims: Array,
  indcs: Vec<usize>,
  line: Array,
  offs: Vec<usize>,
  value: Array,
}

fuzz_target!(|values: Values| {
  let Ok(csl) = CslVec::new(values.dims, values.data, values.indcs, values.offs) else {
    return;
  };

  let _ = csl.line(values.line);

  let _ = csl.value(values.value);

  if let Ok(r) = csl.outermost_line_iter() {
    r.for_each(|_| {});
  } else {
    return;
  };

  if let Ok(r) = csl.outermost_line_rayon_iter() {
    r.enumerate().for_each(|(_, _)| {});
  }
});
