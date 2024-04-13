Holmes
======

This repository was born while I was reading "Introduction to choreographies" by Fabrizio Montesi
drawing its name inspiration from the renowned detective Sherlock Holmes and his outstanding inference gifts.

The initial chapter of the book introduces inference systems, formal mechanisms
that utilize logical rules to derive new information from established facts.
Throughout the chapter, the book provides examples and exercises on this subject.

During the exercise sessions, some solutions are provided, while others remain absent.
Thus, there arises a necessity to validate exercises lacking corresponding solutions.

Initially I thought about using proof assistants like Coq or Matita, but then... where would the fun be?
Rust boasts a sophisticated type system, which is actually a form of proof verifier, it verifies that the
rules of the language hold.

Intrigued, I embark on an exploration to assess its capabilities.
The fundamental concept involves leveraging Rust's compile-time capabilities to encode axioms, deduction rules, and verify theorems.
Any failure in type verification by the compiler indicates a flaw in the inference under validation, ruling out implementation errors.

### Prior art

- [peano (crate)](https://docs.rs/peano/latest/peano/)
- [peano (pages)](https://wg-romank.github.io/peano/)
- [Rust's trait system is a proof engine, let's make it prove us an ABI!](https://youtu.be/g6mUtBVESb0?feature=shared)
