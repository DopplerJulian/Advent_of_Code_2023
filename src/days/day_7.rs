use std::cmp::Ordering;
use std::collections::BTreeMap;
use rayon::prelude::*;

#[derive(PartialEq,Eq, Ord, PartialOrd)]
enum Type{
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}
#[derive(PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: [char; 5],
    c_type: Type,
} impl Hand {
    fn from(hand: &str) -> Self {
        let mut cards= ['-';5];
        hand.char_indices().for_each(|(i,c)| {
            cards[i] = match c {
                c if c.is_digit(10) => c,
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => 'B',
                'T' => 'A',
                _ => panic!("there should be no other Characters"),
            }
        });
        let sets: Vec<(char, u32)> = cards.iter()
            .fold(vec![],|mut acc, e| {
            if let Some((_,count)) = acc.iter_mut().find(|c|c.0 == *e) {
                *count += 1;
            } else {
                acc.push((*e,1))
            }
            acc
        });
        let c_type = match sets {
            s if s.len()==1 => Type::Five,
            s if s.len()==5 => Type::High,
            s if s.len()==4 => Type::One,
            s if s.len()==2 && s.iter().any(|c| c.1==4) => Type::Four,
            s if s.len()==2 && s.iter().any(|c| c.1==3) => Type::Full,
            s if s.iter().any(|c| c.1==3) => Type::Three,
            s if s.len()==3 => Type::Two,
            _ => panic!("There should be no other case"),
        };
        Hand{cards, c_type}
    }

    fn from_p2_rules(hand: &str) -> Self {
        let mut cards= ['-';5];
        hand.char_indices().for_each(|(i,c)| {
            cards[i] = match c {
                c if c.is_digit(10) => c,
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => '0',
                'T' => 'A',
                _ => panic!("there should be no other Characters"),
            }
        });
        let mut sets: Vec<(char, u32)> = cards.iter()
            .fold(vec![],|mut acc, e| {
                if let Some((_,count)) = acc.iter_mut().find(|c|c.0 == *e) {
                    *count += 1;
                } else {
                    acc.push((*e,1))
                }
                acc
            });
        sets.sort_by(|a,b| b.1.cmp(&a.1));
        if let Some(i) = sets.iter().position(|v| v.0 == '0') {
            let count = sets[i].1.to_owned();
            if let Some((_,v)) = sets.iter_mut().find(|v|v.0 != '0') {
                *v += count;
                sets.remove(i);
            }
        }
        let c_type = match sets {
            s if s.len()==1 => Type::Five,
            s if s.len()==5 => Type::High,
            s if s.len()==4 => Type::One,
            s if s.len()==2 && s.iter().any(|c| c.1==4) => Type::Four,
            s if s.len()==2 && s.iter().any(|c| c.1==3) => Type::Full,
            s if s.iter().any(|c| c.1==3) => Type::Three,
            s if s.len()==3 => Type::Two,
            _ => panic!("There should be no other case"),
        };
        Hand{cards, c_type}
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.c_type, self.cards).cmp(&(&other.c_type, other.cards))
    }
}



#[allow(unused)]
pub fn part_1(hands: &str) -> usize {
    let btree: BTreeMap<Hand,usize> = hands.par_lines()
        .map(|line|{
        let split = line.split_once(' ').unwrap();
        let hand = Hand::from(split.0);
        let bid = split.1.parse::<usize>().unwrap();
        (hand, bid)
    }).collect();

    btree.values().enumerate()
        .map(|(pos,bid)| (pos+1)*bid)
        .sum()
}

#[allow(unused)]
pub fn part_2(hands: &str) -> usize {
    let btree: BTreeMap<Hand,usize> = hands.par_lines()
        .map(|line|{
            let split = line.split_once(' ').unwrap();
            let hand = Hand::from_p2_rules(split.0);
            let bid = split.1.parse::<usize>().unwrap();
            (hand, bid)
        }).collect();

    btree.values().enumerate()
        .map(|(pos,bid)| (pos+1)*bid)
        .sum()
}