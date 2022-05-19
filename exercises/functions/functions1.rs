// functions1.rs
// Update get_body so that it works when its called in main()
// and returns a Vec<u8>.
// And make the rest compile!

// I AM NOT DONE

struct Block {
    header: u32,
    details: Vec<u8>,
}

impl Block { 
    fn get_body() -> Vec<u8> {
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

    let word = ??? // TODO
    if is_academy_word() {
        println!("That's definitely a Polkadot Academy word!");
    } else {
        println!("That's not a Polkadot Academy word.");
    }

    println!("{}", block.header );
    println!("{:?}", block.details );
}