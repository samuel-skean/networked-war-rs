// STRETCH: Make it so that an array of moves has an allowable wire format. To
// do this best, most expectedly, I'd want to allow null bytes to appear in the
// padding so that such an array could be made from fresh-from-the-kernel
// memory. That would mean banishing null bytes from at least the start of
// messages, breaking compatibility. Banishing them entirely might seem clean,
// but there's no need to, and that would cramp my style for card comparison.
//
// But, no one says the padding in the array has to be null bytes (I don't
// think...). It would be neat if we allowed some other byte, say 0xff, to
// appear between messages. How does Rust deal with padding caused by
// "differently-sized" enum variants, anyway?
// 
// Also, does Rust even guarantee the layout of enums in the way that I want
// with these `repr` options?
#[repr(u8)]
pub enum Message {
    WantGame = 0,
    GameStart(Hand) = 1,
    PlayCard(Card) = 2,
    PlayResult(RoundResult) = 3,
}


pub type Hand = [Card; 26];

const NUM_CARDS_IN_SUIT: u8 = 13;
const NUM_SUITS: u8 = 4;
const NUM_CARDS_TOTAL: u8 = NUM_CARDS_IN_SUIT * NUM_SUITS;
/// TODO: Document mapping of rank and suit onto value.
#[repr(transparent)]
// STRETCH: Make Card displayable with its human name, too.
//
// SUPER (COOL) STRETCH: Enforce that name's consistency with the constants in
// the test? Macro time? ;)
#[derive(Debug)]
pub struct Card(u8);

#[derive(thiserror::Error, Debug)]
#[error("Card's value was {value}, the maximum is {max}", max = NUM_CARDS_TOTAL - 1)]
pub struct CardValueTooBig {
    value: u8,
}

impl TryFrom<u8> for Card {
    type Error = CardValueTooBig;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= NUM_CARDS_TOTAL {
            Err(CardValueTooBig { value: value })
        } else {
            Ok(Card(value))
        }
    }
}

/// Compares by rank, and nothing else.
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_rank = self.0 % 13;
        let other_rank = other.0 % 13;
        self_rank.cmp(&other_rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for Card {}

// We could use std::cmp::Ordering for this, but then we'd lose the nice
// property of the wire format being the same as the in-memory format.
#[repr(u8)]
pub enum RoundResult {
    Win  = 0,
    Draw = 1,
    Lose = 2,
}

impl From<std::cmp::Ordering> for RoundResult {
    /// This implementation assumes the player we're generating a message for
    /// was the first item in the comparison.
    fn from(value: std::cmp::Ordering) -> Self {
        match value {
            std::cmp::Ordering::Less => Self::Lose,
            std::cmp::Ordering::Equal => Self::Draw,
            std::cmp::Ordering::Greater => Self::Win,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::format::{NUM_CARDS_IN_SUIT, NUM_CARDS_TOTAL, NUM_SUITS};

    use super::Card;

    /// We are dealing with **PLAYING CARDS**.
    /// 
    /// (This is some verbose 'idiot-proof' brainrot, but that's how I'm feeling
    /// rn).
    #[test]
    fn card_constants() {
        assert_eq!(NUM_CARDS_IN_SUIT, 13);
        assert_eq!(NUM_SUITS, 4);
        assert_eq!(NUM_CARDS_TOTAL, 52);
    }
    
    const fn checked_card(value: u8) -> Card {
        assert!(value < NUM_CARDS_TOTAL);
        Card(value)
    }
    const TWO_OF_CLUBS: Card = checked_card(0);
    const THREE_OF_CLUBS: Card = checked_card(1);
    const FOUR_OF_CLUBS: Card = checked_card(2);

    const KING_OF_CLUBS: Card = checked_card(11);
    const ACE_OF_CLUBS: Card = checked_card(12);
    const TWO_OF_DIAMONDS: Card = checked_card(13);
    const THREE_OF_DIAMONDS: Card = checked_card(14);

    const QUEEN_OF_HEARTS: Card = checked_card(36);
    const KING_OF_HEARTS: Card = checked_card(37);
    const ACE_OF_HEARTS: Card = checked_card(38);
    
    const QUEEN_OF_SPADES: Card = checked_card(49);
    const KING_OF_SPADES: Card = checked_card(50);
    const ACE_OF_SPADES: Card = checked_card(51);

    #[test]
    fn card_format() {
        assert_eq!(size_of::<Card>(), 1);
        assert_eq!(align_of::<Card>(), 1);
        assert_eq!(Card::try_from(2 * NUM_CARDS_IN_SUIT + 10).unwrap(), QUEEN_OF_HEARTS);
        assert_eq!(Card::try_from(2 * NUM_CARDS_IN_SUIT + 11).unwrap(), KING_OF_HEARTS);
        assert_eq!(Card::try_from(2 * NUM_CARDS_IN_SUIT + 12).unwrap(), ACE_OF_HEARTS);
    }

    /// This test really only exists because I was gonna write it to test how
    /// the derive macros for (Partial)?(Eq|Ord) work, and I might as well keep
    /// it.
    #[test]
    fn card_comparison() {
        // Duality of PartialOrd
        assert!(TWO_OF_CLUBS < THREE_OF_CLUBS);
        assert!(THREE_OF_CLUBS > TWO_OF_CLUBS);

        assert!(TWO_OF_CLUBS < FOUR_OF_CLUBS);
        assert!(FOUR_OF_CLUBS > TWO_OF_CLUBS);
        
        assert!(THREE_OF_CLUBS < FOUR_OF_CLUBS);
        assert!(FOUR_OF_CLUBS > THREE_OF_CLUBS);

        assert!(THREE_OF_DIAMONDS < QUEEN_OF_SPADES);
        assert!(QUEEN_OF_SPADES > THREE_OF_DIAMONDS);


        // Reflexivity of equality
        assert_eq!(KING_OF_CLUBS, KING_OF_SPADES);
        assert_eq!(KING_OF_SPADES, KING_OF_CLUBS);

        assert_eq!(ACE_OF_CLUBS, ACE_OF_SPADES);
        assert_eq!(ACE_OF_SPADES, ACE_OF_CLUBS);

        assert_eq!(TWO_OF_CLUBS, TWO_OF_DIAMONDS);
        assert_eq!(TWO_OF_DIAMONDS, TWO_OF_CLUBS);

        // Transitivity of equality
        assert_eq!(KING_OF_CLUBS, KING_OF_HEARTS);
        assert_eq!(KING_OF_HEARTS, KING_OF_SPADES);
        assert_eq!(KING_OF_CLUBS, KING_OF_SPADES);

    }

}