use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 7, part 2 result: {}", result);
    Ok(())
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!("{} isn't a valid card", value),
        }
    }
}

fn compare_cards(left: &Vec<Card>, right: &Vec<Card>) -> Option<std::cmp::Ordering> {
    for (i, card) in left.iter().enumerate() {
        if card == &right[i] {
            continue;
        }

        if card > &right[i] {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Less);
        }
    }

    None
}

fn calculate_category(cards: &Vec<Card>) -> Category {
    let mut card_types: HashMap<Card, i64> = HashMap::new();
    let mut joker_count = 0;
    let mut most_frequent_card = Card::J;
    let mut most_frequent_card_count = 0 as i64;
    for card in cards {
        if card == &Card::J {
            joker_count += 1;
            continue;
        }

        if let Some(count) = card_types.get_mut(card) {
            *count += 1;
            if *count > most_frequent_card_count {
                most_frequent_card = card.clone();
                most_frequent_card_count = count.clone();
            }
        } else {
            card_types.insert(card.clone(), 1);
            if 1 > most_frequent_card_count {
                most_frequent_card = card.clone();
                most_frequent_card_count = 1;
            }
        }
    }

    if joker_count == 5 {
        return Category::FiveOfAKind;
    }

    if joker_count > 0 {
        if let Some(cc) = card_types.get_mut(&most_frequent_card) {
            *cc += joker_count;
        }
    }

    match card_types.len() {
        5 => return Category::HighCard,
        4 => return Category::OnePair,
        3 => {
            if card_types.values().any(|v| v == &3) {
                return Category::ThreeOfAKind;
            } else {
                return Category::TwoPair;
            }
        }
        2 => {
            if card_types.values().any(|v| v == &4) {
                return Category::FourOfAKind;
            } else {
                return Category::FullHouse;
            }
        }
        1 => return Category::FiveOfAKind,
        _ => unreachable!("don't know what to do with {} card types", card_types.len()),
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Hand {
    cards: Vec<Card>,
    category: Category,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.category.partial_cmp(&other.category) {
            Some(std::cmp::Ordering::Equal) => compare_cards(&self.cards, &other.cards),
            order => order,
        }
    }
}

#[derive(Eq, Ord)]
struct Round {
    hand: Hand,
    bid: i64,
}

impl From<&String> for Round {
    fn from(value: &String) -> Self {
        let parts = value.split(" ").collect::<Vec<&str>>();
        let cards = parts[0].chars().map(|c| c.into()).collect::<Vec<Card>>();
        let category = calculate_category(&cards);

        Round {
            hand: Hand {
                cards: cards,
                category: category,
            },
            bid: parts[1].parse::<i64>().unwrap(),
        }
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.bid == other.bid
    }
}

fn solve(lines: Vec<String>) -> i64 {
    let mut rounds: Vec<Round> = lines.iter().map(|l| l.into()).collect();
    rounds.sort();

    rounds
        .iter()
        .enumerate()
        .map(|(i, r)| r.bid * (i + 1) as i64)
        .sum::<i64>()
}

fn lines(path: String) -> Result<Vec<String>> {
    let input_data: String = String::from_utf8(std::fs::read(path)?)?;
    let l: Vec<String> = input_data
        .trim()
        .split('\n')
        .map(|input| input.trim().to_string())
        .filter(|input| input != "")
        .collect();
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let expected = 5905;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}
