// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;

pub fn main() {
    // TODO: Implement your guest code here
    // Load the first number from the host
    let a: u64 = env::read();
    // Load the second number from the host
    let b: u64 = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    if a == 1 || b == 1 {
        panic!("Trivial factors")
    }
    // Compute the product while being careful with integer overflow
    let product = a.checked_mul(b).expect("Integer overflow");
    env::commit(&product);
}
