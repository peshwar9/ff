use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
struct FieldElement {
    value: i64,
    prime: i64,
}

impl FieldElement {
    fn new(value: i64, prime: i64) -> Self {
        if value >= prime || value < 0 {
            panic!{" Value not in field range: 0 to {:?}", prime -1};
        }
        FieldElement {value, prime}
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't add numbers in different fields")
        }
        FieldElement::new((self.value + other.value) % self.prime, self.prime)
        
    }
}
fn main() {
    let a = FieldElement::new(7,13);
    let b = FieldElement::new(10,13);
    println!("a + b = {:?}", a.clone() + b.clone());

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field_element_new_valid() {
        let fe = FieldElement::new(7,13);
        assert_eq!(fe.value, 7);
        assert_eq!(fe.prime, 13);
    }

    #[test]
    #[should_panic(expected = " Value not in field range: 0 to 12")]
    fn test_field_invalid_value() {
        FieldElement::new(13,13);
    }

    #[test]
    fn test_field_elements_addition() {
        let fe1 = FieldElement::new(7,13);
        let fe2 = FieldElement::new(6,13);
        assert_eq!(FieldElement::new(0,13), fe1.add(fe2));
    }

    #[test]
    #[should_panic(expected = "Can't add numbers in different fields")]
    fn test_field_element_addition_different_fields() {
        let fe1 = FieldElement::new(7,13);
        let fe2 = FieldElement::new(8,12);
        let _ = fe1.add(fe2);
    }
}
