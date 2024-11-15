# Finite Field Arithmetic in Rust

This Rust implementation provides a basic abstraction for performing arithmetic operations in a finite field (also known as a Galois Field). Finite fields are mathematical structures that play a vital role in cryptography, error correction, and algebraic geometry.

## Features

- **Finite Field Representation**: Encapsulates the finite field element as a struct with a value and a prime modulus.
- **Arithmetic Operations**:
  - Addition: Adds two finite field elements and reduces the result modulo the prime.
  - (Extendable) Framework for additional operations like subtraction, multiplication, division, and exponentiation.

---

## Code Design

### Core Struct

The `FieldElement` struct represents an element in a finite field:

```rust
#[derive(Debug, PartialEq)]
struct FieldElement {
    value: u32,
    prime: u32,
}
```
## Examples
### Creating a finite field
```
use finite_field::FieldElement;

let fe = FieldElement::new(7, 13);
println!("{:?}", fe); // Output: FieldElement { value: 7, prime: 13 }

```
### Adding two finite field elements
```
use finite_field::FieldElement;

let fe1 = FieldElement::new(7, 13);
let fe2 = FieldElement::new(6, 13);
let sum = fe1.add(&fe2);
println!("{:?}", sum); // Output: FieldElement { value: 0, prime: 13 }
```

## Tests
To run the tests
```
cargo test
```

## License
This project is open-source and licensed under the MIT License. Feel free to use, modify, and distribute
