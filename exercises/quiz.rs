// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
// - Tests

// Write a function called `double` that can returns twice the amount
// it takes and returns a u32. Write a second functions called 
// `times_two` that does the same but can handle negative integers.
// Then, complete the tests for them.

// TODO put your functions here!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(8), ???);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        // TODO replace unimplemented!() with an assert for `times_two(-4)`
        unimplemented!()
    }

    // Don't modify this function!
    #[test]
    fn verify_test() {
        let price = double(35);
        assert_eq!(70, price);
    }
}