//! Random CSL

#![no_main]

use arbitrary as _;
use rayon as _;

use libfuzzer_sys::fuzz_target;
use ndstruct::coo::CooVec;
use rand::rngs::mock::StepRng;

fuzz_target!(|values: ([usize; 2], usize)| {
  let (dims, nnz) = values;
  let _rslt = CooVec::new_controlled_random_rand(dims, nnz, &mut StepRng::new(0, 0), |_, _| 0);
});
