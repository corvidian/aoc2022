use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use serde::Deserialize;
use serde_json::Value;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
enum Item {
    Integer(u64),
    List(Vec<Item>),
}

impl<const N: usize> From<[u64; N]> for Item {
    fn from(items: [u64; N]) -> Self {
        Item::List(items.iter().map(|i| i.into()).collect())
    }
}

impl From<&[u64]> for Item {
    fn from(items: &[u64]) -> Self {
        Item::List(items.iter().map(|i| i.into()).collect())
    }
}

impl From<&[Item]> for Item {
    fn from(items: &[Item]) -> Self {
        Item::List(items.to_vec())
    }
}

impl From<u64> for Item {
    fn from(item: u64) -> Self {
        Item::Integer(item)
    }
}

impl From<&u64> for Item {
    fn from(item: &u64) -> Self {
        Item::Integer(*item)
    }
}

impl From<Value> for Item {
    fn from(item: Value) -> Self {
        match item {
            Value::Array(values) => Item::List(values.into_iter().map(|i| i.into()).collect()),
            Value::Number(num) if num.is_u64() => num.as_u64().unwrap().into(),
            a => unimplemented!("{a}"),
        }
    }
}

impl From<&str> for Item {
    fn from(item: &str) -> Self {
        serde_json::from_str::<Value>(item).unwrap().into()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        fn cmp_list(left: &[Item], right: &[Item]) -> Ordering {
            for pair in left.iter().zip_longest(right.iter()) {
                match pair {
                    Both(a, b) => match a.cmp(b) {
                        Ordering::Equal => {}
                        other => return other,
                    },
                    Left(_) => return Ordering::Greater,
                    Right(_) => return Ordering::Less,
                }
            }
            Ordering::Equal
        }

        match (self, other) {
            (Item::Integer(a), Item::Integer(b)) => a.cmp(b),
            (Item::List(a), Item::List(b)) => cmp_list(a, b),
            (Item::Integer(a), Item::List(b)) => cmp_list(&[a.into()], b),
            (Item::List(a), Item::Integer(b)) => cmp_list(a, &[b.into()]),
        }
    }
}

fn main() {
    let input = aoc::read_input_lines();
    let pairs: Vec<_> = input
        .chunks(3)
        .map(|pair| {
            pair.iter()
                .take(2)
                .map(|str| Item::from(str.as_str()))
                .collect::<Vec<_>>()
        })
        .collect();

    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| (pair[0] < pair[1]).then_some(i + 1))
        .sum();
    println!("Part 1: {sum}");

    let divisors: [Item; 2] = ["[[2]]".into(), "[[6]]".into()];

    let product: usize = pairs
        .into_iter()
        .flat_map(|pair| pair.into_iter())
        .chain(divisors.iter().cloned())
        .sorted()
        .enumerate()
        .filter_map(|(i, item)| divisors.contains(&item).then_some(i + 1))
        .product();
    println!("Part 2: {}", product);
}

#[cfg(test)]
mod tests {

    use super::Item;

    #[test]
    fn item_compare_two_lists_of_same_length() {
        let item1: Item = "[1,1,3,1,1]".into();
        let item2: Item = "[1,1,5,1,1]".into();

        assert!(item1 < item2)
    }

    #[test]
    fn item_compare_nested_things() {
        let item1: Item = "[[1],[2,3,4]]".into();
        let item2: Item = "[[1],4]".into();

        assert!(item1 < item2)
    }

    #[test]
    fn item_compare_different_types() {
        let item1: Item = Item::from("[9]");
        let item2: Item = Item::from("[[8,7,6]]");

        assert!(item1 > item2)
    }

    #[test]
    fn item_compare_list_different_lengths() {
        let item1 = Item::from("[[4,4],4,4]");
        let item2 = Item::from("[[4,4],4,4,4]");

        assert!(item1 < item2)
    }

    #[test]
    fn item_compare_list_different_lengths_reversed() {
        let item1 = Item::from([7, 7, 7, 7]);
        let item2 = Item::from([7, 7, 7]);

        assert!(item1 > item2)
    }

    #[test]
    fn item_compare_empty_list() {
        let item1 = Item::from([]);
        let item2 = Item::from([3]);

        assert!(item1 < item2)
    }

    #[test]
    fn item_compare_nested_empty_list() {
        let item1 = Item::from("[[[]]]");
        let item2 = Item::from("[[]]");

        assert!(item1 > item2)
    }

    #[test]
    fn item_compare_longer_lists() {
        let item1 = Item::from("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let item2 = Item::from("[1,[2,[3,[4,[5,6,0]]]],8,9]");

        assert!(item1 > item2)
    }
}
