use ormen::*;

pub mod lib;

fn main() {
    println!("Hello, world!");
    let deck: Vec<Card> = get_shuffled_deck();
    print_deck(deck);
}
