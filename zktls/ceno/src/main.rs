extern crate ceno_rt;
use ceno_rt::debug_println;
use std::fmt::Write;
use rkyv::Archived;

fn main() {
    debug_println!("cycle-tracker-start: tls");
    //sp1_zkvm::io::read_vec();
    let input: &Archived<Vec<u8>> = ceno_rt::read();
    debug_println!("size of input: {}", input.len());
    let output = zktls_replayable_tls::entry(&input);
    debug_println!("size of output: {}", output.len());
    //ceno_rt::commit::<Archived<Vec<u8>>, _>(&output);
    //sp1_zkvm::io::commit_slice(&output);
    debug_println!("cycle-tracker-end: tls");
}
