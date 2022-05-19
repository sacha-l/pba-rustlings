// event_enums
// Make me compile! Execute `rustlings hint event_enums` for hints!

// I AM NOT DONE

#[derive(Debug)]
enum Event {
    // have a look at the main() function to include the 
    // appropriate events and types
}

impl Event {
    // add a function that prints the events to the console
}

fn main() {
    let events = [        
        Event::Instantiated(String::from("ContractName")),
        Event::Transferred { to: 10, from: 30, balance: u128::MAX },
        Event::Terminated,
    ];

    for event in &events {
        event.call();
    }
}