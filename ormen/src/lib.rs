use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{cmp::min, fmt, io};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: usize,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = vec!['♥', '♦', '♣', '♠'][self.suit as usize];
        let mut stringval = " ".to_owned();
        stringval.push_str(&self.value.to_string());

        let value = match self.value {
            1 => " A",
            10 => "10",
            11 => " J",
            12 => " D",
            13 => " K",
            _ => &stringval,
        };
        write!(f, "|{}{}|", suit, value)
    }
}

fn get_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for suit_id in 0..4 {
        let suit: Suit = match suit_id {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            _ => Suit::Spades,
        };
        for value in 1..14 {
            deck.push(Card {
                suit: suit.clone(),
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
    pub fn new() -> Game {
        Game {
            deck: get_shuffled_deck(),
            snake: vec![],
            played: vec![],
        }
    }

    pub fn play(&mut self, player: Player) {
        while self.deck.len() != 0 {
            // TODO: Implement turn
            self.snake.push(self.deck.pop().unwrap());
            // player.take_turn(vec![self.snake.clone()], self.played.clone());
            let mut guess = String::new();
            let _ = io::stdin().read_line(&mut guess);
            Self::print_board(&self);
          
            self.play_round(
                self.snake.clone(), 
                self.snake.len() - 1, 
                self.snake.len() - 1
            );
            //player.take_turn(vec![self.snake.clone()], self.played.clone());
        }
    }

    fn play_round(&mut self, snake: Vec<Card>, index: usize, last_index: usize) -> Option<Vec<Vec<Card>>> {
        // look all 3 cards behind last pos, look 1 card behind current pos, look at current pos
        let len = snake.len();
        
        for i in 1..4 {
            if (last_index + i > len - 1) {
                self.check_merge(snake.clone(), last_index + i);
            }
        }

        if (index + 1 > len-1) {
            self.check_merge(snake.clone(), index + 1);
        }

        self.check_merge(snake.clone(), index);

        None
    }

    fn check_merge(&mut self, snake: Vec<Card>, index: usize) -> Option<Vec<Vec<Card>>> {
        if (index > 0) {
            if (snake[index].suit == snake[index - 1].suit || snake[index].value == snake[index-1].value) {
                self.play_round(snake.clone(), index - 1, index);
            }
        }
        if (index > 2) {
            if (snake[index].suit == snake[index - 3].suit || snake[index].value == snake[index-3].value) {
                self.play_round(snake.clone(), index - 3, index);
            }
        }

        None
    }

    fn print_board(&self) {
        print!("{}[2J", 27 as char);
        let len = self.snake.len();
        Self::print_board_row(self.snake.clone(), 0, min(26, len));
        Self::print_board_row(self.snake.clone(), 26, len);
    }

    fn print_board_row(snake: Vec<Card>, start: usize, end: usize) {
        for _ in start..end {
            print!(" --- ");
        }
        println!();
        for i in start..end {
            let card = snake[i];
            print!("|{}  |", Self::get_suit_char(card.suit));
        }
        println!();
        for _ in start..end {
            print!("|   |");
        }
        println!();
        for i in start..end {
            let card = snake[i];
            print!("| {}|", Self::get_value_string(card.value));
        }
        println!();
        for _ in start..end {
            print!(" --- ");
        }
        println!();
    }

    fn get_value_string(value: usize) -> String {
        let mut svalue = " ".to_owned();
        svalue.push_str(&value.to_string());
        match value {
            1 => " A",
            10 => "10",
            11 => " J",
            12 => " D",
            13 => " K",
            _ => &svalue,
        }
        .to_string()
    }

    fn get_suit_char(suit: Suit) -> char {
        vec!['♥', '♦', '♣', '♠'][suit as usize]
    }
}

// pub trait Behaviour {
//     fn take_turn(&self, snakes: Vec<Vec<Card>>, played: Vec<Card>) -> usize;
// }
pub struct Player {
    pub wins: usize,
    pub losses: usize,
}
impl Player {
    pub fn take_turn(&self, snakes: Vec<Vec<Card>>, played: Vec<Card>) -> usize {
        0
    }
}
