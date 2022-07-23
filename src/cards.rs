#[derive(Debug, PartialEq)]
pub enum CardError {
    NoCardsLeft,
}

#[derive(Debug, PartialEq)]
pub struct Cards {
    in_hand: u8,
    discarded: u8,
    lost: u8,
}

impl Cards {
    pub fn new(starting_hand: u8) -> Self {
        Self { in_hand: starting_hand, discarded: 0, lost: 0 }
    }

    pub fn play_round(&mut self, burned_cards: u8) -> Result<(), CardError> {
        match &self {
            Self { in_hand: 2..=u8::MAX, .. } => {
                self.in_hand -= 2;
                self.discarded += 2;
                self.lost += burned_cards;
                Ok(())
            },
            Self { in_hand: 1, discarded: 2..=u8::MAX, ..} | Self { in_hand: 0, discarded: 3..=u8::MAX, ..} => {
                self.in_hand = &self.in_hand + &self.discarded - burned_cards - 3;
                self.discarded = 2;
                self.lost = self.lost + burned_cards + 1;
                Ok(())
            },
            Self { .. } => Err(CardError::NoCardsLeft),
        }
    }

    pub fn calculate_maximum_rounds(&self) -> u8 {
        let mut cards = Self {..*self};
        let mut turns = 0;
        while let Ok(()) = cards.play_round(0) {
            turns += 1;
        }

        turns
    }
}

impl std::fmt::Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🃏:{}, 🗑️:{}, 🔥:{}", self.in_hand, self.discarded, self.lost)
    }
} 

#[cfg(test)]
mod test {
    // use super::*;

    use crate::{Cards, cards::CardError};

    #[test]
    fn play_round_with_no_cards() {
        let mut cards = Cards {
            in_hand: 0,
            discarded: 2,
            lost: 8,
        };
        assert_eq!(cards.play_round(0), Err(CardError::NoCardsLeft));
    }

    #[test]
    fn play_round_burn_cards() {
        let mut cards = Cards::new(10);
        let _ = cards.play_round(2);
        assert_eq!(cards.lost, 2);
    }

    #[test]
    fn calculate_maximum_rounds_no_cards() {
        let cards = Cards::new(0);
        assert_eq!(cards.calculate_maximum_rounds(), 0);
    }

    #[test]
    fn calculate_maximum_rounds_10_cards() {
        let cards = Cards::new(10);
        assert_eq!(cards.calculate_maximum_rounds(), 25);
    }

}