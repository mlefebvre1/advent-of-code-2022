use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, fs, str::FromStr};

fn main() -> Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

fn first() -> Result<String> {
    let data = fs::read_to_string("day7/data/day7.txt")?;
    let file_and_paths = get_all_paths(&data)?;
    let directories: HashSet<&String> = file_and_paths.iter().map(|(_, path)| path).collect();
    let directories_size = calculate_size_of_directories(&directories, &file_and_paths);
    let ans: usize = directories_size.filter(|&size| size <= 100000).sum();

    Ok(ans.to_string())
}

fn second() -> Result<String> {
    const AVAILABLE_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;

    let data = fs::read_to_string("day7/data/day7.txt")?;
    let file_and_paths = get_all_paths(&data)?;
    let directories: HashSet<&String> = file_and_paths.iter().map(|(_, path)| path).collect();
    let directories_size = calculate_size_of_directories(&directories, &file_and_paths)
        .sorted()
        .collect_vec();

    let space_left = AVAILABLE_SPACE - directories_size.iter().max().unwrap();
    let space_needed = REQUIRED_SPACE - space_left;

    // Find the directory with the size bigger than `space_needed` and the closest to `space_needed`
    let (ans, _) = directories_size
        .into_iter()
        .filter(|&size| size > space_needed)
        .map(|size| (size, size - space_needed))
        .min_by_key(|x| x.1)
        .unwrap();
    Ok(ans.to_string())
}

fn get_all_paths(data: &str) -> Result<Vec<(usize, String)>> {
    let mut pwd = Vec::new();
    let mut file_and_paths = Vec::new();
    for line in data.lines() {
        match Entry::from_str(line)? {
            Entry::Command(Command::Cd(dir)) => {
                apply_cd(&dir, &mut pwd);
            }
            Entry::Dir => {
                let path = pwd.join("/").replace("//", "/");
                file_and_paths.push((0, path))
            }
            Entry::File(file_size) => {
                let path = pwd.join("/").replace("//", "/");
                file_and_paths.push((file_size, path))
            }
            _ => (),
        }
    }
    Ok(file_and_paths)
}

fn apply_cd(next_dir: &str, tree_path: &mut Vec<String>) {
    if next_dir == ".." {
        tree_path.pop();
    } else {
        tree_path.push(next_dir.to_owned());
    }
}

fn calculate_size_of_directories<'a>(
    directories: &'a HashSet<&'a String>,
    all_files: &'a [(usize, String)],
) -> impl Iterator<Item = usize> + 'a {
    directories.iter().map(|&directory| {
        let files_of_directory = all_files
            .iter()
            .filter(|(_, file)| file.starts_with(directory));
        let total_size: usize = files_of_directory.map(|(file_size, _)| file_size).sum();
        total_size
    })
}

enum Command {
    Cd(String),
    Ls,
}
impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with("cd") => {
                let dir = s.trim_start_matches("cd ");
                Ok(Self::Cd(dir.to_string()))
            }
            s if s.starts_with("ls") => Ok(Self::Ls),
            _ => Err(anyhow!("Command not supported")),
        }
    }
}

enum Entry {
    Command(Command),
    Dir,
    File(usize),
}
impl FromStr for Entry {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with('$') => {
                let s = s.trim_start_matches("$ ");
                Ok(Self::Command(Command::from_str(s)?))
            }
            s if s.starts_with("dir") => Ok(Self::Dir),
            _ => {
                let mut s = s.split_whitespace();
                let file_size: usize = s.next().unwrap().parse()?;
                Ok(Self::File(file_size))
            }
        }
    }
}
