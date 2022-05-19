// iterators1.rs
//
//  Make me compile by filling in the `???`s and notice some other issues with the
//  code.
//
// Execute `rustlings hint iterators1` for hints :D

fn main () {
    let my_fav_things = vec!["runtime", "libraries", "avocado", "substrate", "cryptography"];

    let mut my_iterable_fav_things = ???;

    assert_eq!(my_iterable_fav_things.next(), ???);
    assert_eq!(my_iterable_fav_things.next(), Some("libraries"));
    assert_eq!(my_iterable_fav_things.next(), ???);
    assert_eq!(my_iterable_fav_things.next(), Some("substrate"));
    assert_eq!(my_iterable_fav_things.next(), Some("cryptography"));
    assert_eq!(my_iterable_fav_things.next(), ???);
}
