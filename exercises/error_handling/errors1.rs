// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions.
// Make sure all tests pass.

// I AM NOT DONE

pub fn generate_nametag_text(name: String) -> String {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        Err(404)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test passes initially if you comment out the 2nd test.
    // You'll need to update what this test expects when you change
    // the function under test!
    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            Err(404)
        );
    }
}