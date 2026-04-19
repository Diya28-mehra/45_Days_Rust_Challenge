use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;


#[derive(Debug, Clone, Copy)]
enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy)]
enum Rank{
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct Card{
    suit: Suit,
    rank: Rank,
}

fn create_deck()->Vec<Card>{
    let suits = [Suit::Hearts,Suit::Spades,Suit::Diamonds,Suit::Clubs];
    let mut deck = Vec::new();

    for &suit in &suits{
        for rank in 2..=10{
            deck.push(Card{
                suit: suit.clone(),
                rank: Rank::Number(rank),
            });
            deck.push(Card{suit:suit.clone(),rank: Rank::Jack});
            deck.push(Card{suit:suit.clone(),rank: Rank::Queen});
            deck.push(Card{suit:suit.clone(),rank: Rank::King});
            deck.push(Card{suit:suit.clone(),rank: Rank::Ace});
        }
    }
    deck
}

fn shuffle_deck(deck: &mut Vec<Card>){
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
}


fn card_value(card: &Card)->u8{
    match card.rank{
        Rank::Number(n)=>n,
        Rank::Jack=>10,
        Rank::Queen=>10,
        Rank::King=>10,
        Rank::Ace=>1,
    }
}

fn display_card(card: &Card) -> String {
    let rank = match card.rank {
        Rank::Number(n) => n.to_string(),
        Rank::Jack => "J".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::King => "K".to_string(),
        Rank::Ace => "A".to_string(),
    };

    let suit = match card.suit {
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
        Suit::Spades => "♠",
    };

    format!("{}{}", rank, suit)
}

fn main() {
    let mut deck = create_deck();
    shuffle_deck(&mut deck);

    println!("🎮 Welcome to High Card Game!");
    println!("Press Enter to draw a card...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let player_card = deck.pop().unwrap();
    let computer_card = deck.pop().unwrap();
    
    println!("You drew: {:?}", display_card(&player_card));
    println!("Computer drew: {:?}", display_card(&computer_card));

    let player_value = card_value(&player_card);
    let computer_value = card_value(&computer_card);

    if player_value>computer_value{
        println!("🎉 You win!");
    }
    else if player_value<computer_value{
        println!("💻 Computer wins!");
    }
    else{
        println!("🤝 It's a tie!");
    }
}


