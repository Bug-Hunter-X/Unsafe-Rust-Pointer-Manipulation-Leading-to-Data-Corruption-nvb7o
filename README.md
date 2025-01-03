# Unsafe Rust Pointer Manipulation

This repository demonstrates a common error in unsafe Rust code involving raw pointers and mutable references. The `bug.rs` file contains code that modifies memory directly using a raw pointer, leading to data corruption. The `bugSolution.rs` file provides a corrected version that uses safe Rust techniques to achieve the desired outcome.