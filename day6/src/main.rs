use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

#[allow(dead_code)]
fn first() -> Result<String> {
    let data = fs::read_to_string("day6/data/day6.txt")?;
    let ans = find_start_of_packet(&data, 4).unwrap();
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let data = fs::read_to_string("day6/data/day6.txt")?;
    let ans = find_start_of_packet(&data, 14).unwrap();
    Ok(ans.to_string())
}
fn find_start_of_packet(data: &str, nb_distinct_chars: usize) -> Option<usize> {
    data.char_indices().find_map(|(i, _)| {
        let set: HashSet<char> = data.chars().skip(i).take(nb_distinct_chars).collect();
        if set.len() == nb_distinct_chars {
            Some(i + nb_distinct_chars)
        } else {
            None
        }
    })
}
