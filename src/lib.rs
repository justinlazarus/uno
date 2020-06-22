mod game {
  mod deck {

  }

  mod player {

  }

  mod hand {

  }

}

const DECK_CARD_COUNT: u8 = 108;
const HIGHEST_NUMBER_CARD: u8 = 9;

enum CardColor {
    red,
    yellow,
    green,
    blue,
}

enum ActionType {
    skip,
    draw_two,
    reverse
}

pub struct ActionCard {
  pub color: CardColor,
  pub action: ActionType
}

pub struct NumberCard {
    pub color: CardColor,
    pub number: u8,
}

fn build_deck() {
    let deck: Vec<u

}