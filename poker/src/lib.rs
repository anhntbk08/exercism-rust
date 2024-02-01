#[cfg(test)]
use std::{println as info, println as warn};

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    error::Error,
    str::FromStr,
};

/// Given a list of poker hands, return a list of those hands which win.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.is_empty() {
        return vec![];
    }
    let mut hands: BinaryHeap<(HandValue, &str)> = hands
        .iter()
        .map(|&s| (HandValue::new(s.parse().unwrap()), s))
        .collect();

    let (winning, s) = hands.pop().unwrap();
    let mut result = vec![s];
    while let Some((value, s)) = hands.pop() {
        if value < winning {
            break;
        }
        result.push(s);
    }
    result
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Value(u8);

impl FromStr for Value {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = {
            match s.parse::<u8>() {
                Ok(v) => v,
                Err(_) => match s {
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "A" => 14,
                    _ => return Err("invalid card value".into()),
                },
            }
        };
        Ok(Value(value))
    }
}

#[derive(Debug)]
struct PokerHand {
    values: [Value; 5],
    suits: [u8; 5],
    group_lengths: Vec<u8>,
    group_values: Vec<Value>,
}

impl PokerHand {
    fn new(values: [Value; 5], suits: [u8; 5]) -> Self {
        let mut groups = HashMap::<Value, u8>::new();
        for &v in values.iter() {
            *groups.entry(v).or_default() += 1;
        }
        let mut groups: Vec<_> = groups.into_iter().collect();
        groups.sort_by_key(|&(v, len)| Reverse((len, v)));
        let (group_values, group_lengths) = groups.into_iter().unzip();
        Self {
            values,
            suits,
            group_lengths,
            group_values,
        }
    }

    fn is_straight(&self) -> Option<Value> {
        if self.group_lengths.len() == 5 {
            if self.values[0].0 == self.values[4].0 + 4 {
                Some(self.values[0])
            } else if self.values[0].0 == 14 && self.values[1].0 == 5 {
                Some(self.values[1])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_flush(&self) -> Option<[Value; 5]> {
        self.suits[1..5]
            .iter()
            .all(|x| x == &self.suits[0])
            .then(|| self.values)
    }
}

impl FromStr for PokerHand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand: Vec<(Value, u8)> = s
            .split_whitespace()
            .map(|card| {
                let i = card.len() - 1;
                (card[0..i].parse().unwrap(), card.as_bytes()[i])
            })
            .collect();
        assert_eq!(hand.len(), 5);
        hand.sort();
        let (values, suits): (Vec<_>, Vec<_>) = hand.into_iter().rev().unzip();
        Ok(PokerHand::new(
            values.try_into().unwrap(),
            suits.try_into().unwrap(),
        ))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandValue {
    HighCard([Value; 5]),
    OnePair([Value; 4]),
    TwoPair([Value; 3]),
    ThreeOfAKind([Value; 3]),
    Straight(Value),
    Flush([Value; 5]),
    FullHouse([Value; 2]),
    FourOfAKind([Value; 2]),
    StraightFlush(Value),
}

impl HandValue {
    fn new(hand: PokerHand) -> Self {
        match (hand.is_straight(), hand.is_flush()) {
            (Some(v), Some(_)) => Self::StraightFlush(v),
            (Some(v), None) => Self::Straight(v),
            (None, Some(v)) => Self::Flush(v),
            (None, None) => {
                let v = hand.group_values;
                match *hand.group_lengths {
                    [4, 1] => Self::FourOfAKind(v.try_into().unwrap()),
                    [3, 2] => Self::FullHouse(v.try_into().unwrap()),
                    [3, 1, 1] => Self::ThreeOfAKind(v.try_into().unwrap()),
                    [2, 2, 1] => Self::TwoPair(v.try_into().unwrap()),
                    [2, 1, 1, 1] => Self::OnePair(v.try_into().unwrap()),
                    _ => Self::HighCard(hand.values),
                }
            }
        }
    }
}