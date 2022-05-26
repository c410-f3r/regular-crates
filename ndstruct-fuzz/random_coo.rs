//! Random COO

#![no_main]

use arbitrary as _;
use rayon as _;

use libfuzzer_sys::fuzz_target;
use ndstruct::csl::CslVec;
use rand::rngs::mock::StepRng;

fuzz_target!(|values: ([usize; 2], usize)| {
  let (dims, nnz) = values;
  let _ = CslVec::new_controlled_random_rand(dims, nnz, &mut StepRng::new(0, 0), |_, _| 0);
});
