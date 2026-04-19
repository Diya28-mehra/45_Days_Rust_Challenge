#[derive(Debug)]
enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
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

fn main() {
    let chance1 = Card{
        suit: Suit::Hearts,
        rank: Rank::Number(7),
    };

    let chance2 = Card{
        suit: Suit::Spades,
        rank: Rank::Ace,
    };

    println!("Chance1 is: {:#?}", chance1);
    println!("Chance2 is: {:#?}", chance2);
}
