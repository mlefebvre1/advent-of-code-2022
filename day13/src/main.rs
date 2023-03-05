use anyhow::Result;
use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs;
use std::iter::zip;

fn main() -> Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

fn first() -> Result<String> {
    let data = fs::read_to_string("day13/data/day13.txt")?;
    let lines: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();
    let ans: usize = lines
        .chunks(2)
        .enumerate()
        .filter_map(|(index, line)| {
            let mut lists = line.iter();
            let list1: List = List(serde_json::from_str(lists.next().unwrap()).unwrap());
            let list2: List = List(serde_json::from_str(lists.next().unwrap()).unwrap());
            match list1.cmp(&list2) {
                Ordering::Less => Some(index + 1),
                _ => None,
            }
        })
        .sum();

    Ok(ans.to_string())
}

fn second() -> Result<String> {
    let divider_packets = ["[[2]]", "[[6]]"];

    let data = fs::read_to_string("day13/data/day13.txt")?;
    let mut lines: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();
    lines.extend(divider_packets); // add the new divider packets

    let lists = lines
        .iter()
        .map(|line| {
            let list: List = List(serde_json::from_str(line).unwrap());
            list
        })
        .sorted();

    let ans: usize = lists
        .enumerate()
        .filter_map(|(index, list)| {
            if list.is_a_divider_packet() {
                Some(index + 1)
            } else {
                None
            }
        })
        .product();

    Ok(ans.to_string())
}

#[derive(Debug)]
struct List(Value);

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        List::compare(&self.0, &other.0) == Ordering::Equal
    }
}
impl Eq for List {}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(List::compare(&self.0, &other.0))
    }
}
impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl List {
    pub fn is_a_divider_packet(&self) -> bool {
        self == &List(serde_json::from_str("[[2]]").unwrap())
            || self == &List(serde_json::from_str("[[6]]").unwrap())
    }
    fn compare(item1: &Value, item2: &Value) -> Ordering {
        /*
            *   If both values are integers, the lower integer should come first.
                If the left integer is lower than the right integer, the inputs are in the right order.
                If the left integer is higher than the right integer, the inputs are not in the right order.
                Otherwise, the inputs are the same integer; continue checking the next part of the input.

            *   If both values are lists, compare the first value of each list, then the second value, and so on.
                If the left list runs out of items first, the inputs are in the right order.
                If the right list runs out of items first, the inputs are not in the right order.
                If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.

            *   If exactly one value is an integer, convert the integer to a list which contains that integer as its only value,
                then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2);
                the result is then found by instead comparing [0,0,0] and [2].
        */
        match (item1, item2) {
            (Value::Array(arr1), Value::Array(arr2)) => {
                for (item1, item2) in zip(arr1.iter(), arr2.iter()) {
                    match List::compare(item1, item2) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                arr1.len().cmp(&arr2.len())
            }
            (Value::Array(arr1), Value::Number(n2)) => {
                let arr1 = Value::Array(arr1.to_vec());
                let arr2 = Value::Array(vec![Value::Number(n2.to_owned())]);
                List::compare(&arr1, &arr2)
            }
            (Value::Number(n1), Value::Array(arr2)) => {
                let arr1 = Value::Array(vec![Value::Number(n1.to_owned())]);
                let arr2 = Value::Array(arr2.to_vec());
                List::compare(&arr1, &arr2)
            }
            (Value::Number(n1), Value::Number(n2)) => {
                let n1 = n1.as_u64().unwrap();
                let n2 = n2.as_u64().unwrap();
                n1.cmp(&n2)
            }
            (Value::Null, Value::Null) => Ordering::Equal,
            (Value::Null, _) => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}
