use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// Inherint Implmentation
impl Deck {

    fn new() -> Self {
        let mut cards = vec![];
        // List of 'suits' - "hearts", "spades"...
        let suits = ["Hearts", "Spades", "Diamonds"];
        // List of 'values' - "ace", "two", "tree"...
        let values = ["Ace", "Two", "Three"];

        // Iterate over suits and values:
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // implicit return
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("Heres your deck: {:#?}", deck);
    // deck.shuffle();
    // FIX ME Add error handling
    let cards = deck.deal(3);
    println!("Heres your hand: {:#?}", cards);
}
