// functions1.rs
// Make me compile without changing fn main()!

// I AM NOT DONE

struct Block {
    header: u32,
    details: Vec<u8>,
}

impl Block { 
    fn get_body(a: String) -> Vec<u8> {
    // TODO
    }
}

fn is_academy_word(attempt: &str) -> bool {
    attempt == "cryptography" || attempt == "economics" || attempt == "genesis block"
}

fn main() {

    let block = Block {
        header: 12345,
        details: Block::get_body("state root")
    };

    println!(r#"     ┌──────────────────┐         "#);
    println!(r#"     │  Block Header    │         "#);
    println!(r#"      ──────────────────          "#);
    println!(r#"     │      {}       │         "#, block.header );
    println!(r#"     │   {}     │         "#, block.details );
    println!(r#"     |                  | "#);
    println!(r#"      ──────────────────  "#);

    // TODO
    if is_academy_word(&word) {
        println!("That's definitely a Polkadot Academy word!");
    } else {
        println!("That's not a Polkadot Academy word I know of.");
    }
}