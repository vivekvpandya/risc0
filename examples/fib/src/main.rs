// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use fib_methods::FIB_ID;
use fib_methods::FIB_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

pub fn fib(a: u64) -> (Receipt, u64) {
    let start = std::time::Instant::now();
    let env = ExecutorEnv::builder()
        // Send a to the guest
        .write(&a)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, FIB_ELF).unwrap();

    let end = std::time::Instant::now();

    println!("Total time for exe + proving {:?}", end - start);
    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    // println!("I know the factors of {}, and I can prove it!", c);

    (receipt, c)
}

fn main() {
    // Pick two numbers
    let (receipt, _) = fib(1000);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(FIB_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
