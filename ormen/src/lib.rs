use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: usize,
}

fn get_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for suit_id in 0..4 {
        for value in 1..14 {
            let suit: Suit = match suit_id {
                0 => Suit::Hearts,
                1 => Suit::Diamonds,
                2 => Suit::Clubs,
                _ => Suit::Spades,
            };
            deck.push(Card {
                suit: suit,
                value: value,
            });
        }
    }

    deck
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
}

pub fn get_shuffled_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = get_deck();
    shuffle_deck(&mut deck);
    deck
}

pub fn print_deck(deck: &Vec<Card>) {
    println!("Number of cards in deck: {:?}", deck.len());
    for card in deck {
        println!("{:?}", card);
    }
}

pub struct Game {
    deck: Vec<Card>,
    snake: Vec<Card>,
    played: Vec<Card>,
}

impl Game {
    pub fn new(&mut self) {
        self.deck = get_shuffled_deck();
        self.snake = vec![];
        self.played = vec![];
    }

    pub fn play(&mut self, player: &impl Behaviour) {
        while self.deck.len() != 0 {
            // TODO: Implement turn
            self.snake.push(self.deck.pop().unwrap());
            player.take_turn(vec![self.snake.clone()], self.played.clone());
        }
    }
}

pub trait Behaviour {
    fn take_turn(&self, snakes: Vec<Vec<Card>>, played: Vec<Card>) -> usize;
}
pub struct Player {
    pub wins: usize,
    pub losses: usize,
}
impl Behaviour for Player {
    fn take_turn(&self, snakes: Vec<Vec<Card>>, played: Vec<Card>) -> usize {
        0
    }
}
