use std::fmt::{Display, Formatter};
use rand::seq::SliceRandom;
use rand::thread_rng;
use Suit::{*};
use Rank::{*};
use Royalty::{*};

const SUITS: [Suit; 4] = [Hearts, Diamonds, Clubs, Spades];
const RANKS: [Rank; 13] = [Num(2),Num(3),Num(4),Num(5),Num(6),Num(7),Num(8),Num(9),Num(10),Face(Jack),Face(Queen),Face(King),Ace];

fn main() {
    let mut deck = Deck::new();
    let mut dealer = Player::new("Dealer".to_owned());
    let mut player = Player::new("Adnug".to_owned());

    for _ in 0..2 {
        deck.deal(&mut player);
        deck.deal(&mut dealer);
    }

    dealer.print_hand();
    player.print_hand();
}

struct Player {
    name: String,
    hand: Vec<Card>
}

impl Player {
    fn new(name: String) -> Self {
        Player { name, hand: Vec::new() }
    }

    fn fmt_hand(&self) -> String {
        self.hand.iter().map(|c| c.to_string()).collect()
    }

    fn print_hand(&self) {
        println!("{} {}", self.name, self.fmt_hand());
    }

    fn take_card(&mut self, card: Card) {
        self.hand.push(card);
    }
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        let cards_to_add = SUITS
            .iter()
            .flat_map(|suit| RANKS.iter().map(|rank| Card::new(*suit, *rank)));

        cards.extend(cards_to_add);
        cards.shuffle(&mut thread_rng());

        Deck { cards }
    }

    fn deal(&mut self, player: &mut Player) {
        player.take_card(self.cards.pop().expect("Out of cards"));
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Card {
    suit: Suit,
    rank: Rank
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}{}]", self.suit, self.rank)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Hearts => { write!(f, "\u{2665}") }
            Diamonds => { write!(f, "\u{2666}") }
            Clubs => { write!(f, "\u{2663}") }
            Spades => { write!(f, "\u{2660}") }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Rank {
    Num(u8),
    Face(Royalty),
    Ace
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Num(10) => { write!(f, "T") }
            Num(num) if (2..=10).contains(num) => { write!(f, "{}", num) }
            Num(num) => { panic!("Unknown rank {}", num); }
            Face(royalty) => { write!(f, "{}", royalty.to_string()) }
            Ace => { write!(f, "A") }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Royalty {
    Jack,
    Queen,
    King
}

impl Display for Royalty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Jack => { write!(f, "J") }
            Queen => { write!(f, "Q") }
            King => { write!(f, "K") }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn should_have_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn should_have_unique_cards() {
        let deck = Deck::new();
        let unique_cards: HashSet<_> = deck.cards.into_iter().collect();
        assert_eq!(unique_cards.len(), 52);
    }

    #[test]
    fn should_print_card() {
        let card = Card::new(Hearts, Face(Queen));
        let card_string = card.to_string();
        assert_eq!(card_string, "[\u{2665}Q]")
    }
}



