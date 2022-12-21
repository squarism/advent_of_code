#![allow(unused)]
use indoc::indoc;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse(input);
    let tree = to_file_tree(parsed);
    let sizes = dir_size(tree);

    let mut total = 0;
    for item in sizes {
        if item.size < 100_000 {
            total += item.size
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }

    // ugh i do NOT want to write a parser
    #[test]
    fn test_parsing() {
        // indoc lets you do heredocs without indenting weirdly etc
        let input = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        "};

        let result = parse(input);

        // there should be a better test but ...
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_to_file_tree() {
        let parsed: Vec<Operation> = vec![
            Operation::DirectoryChange(DirectoryChange {
                destination: "/".to_owned(),
            }),
            Operation::LsCommand(LsCommand {}),
            Operation::File(File {
                name: "b.txt".to_owned(),
                size: 120_000,
            }),
            Operation::File(File {
                name: "c.dat".to_owned(),
                size: 9001,
            }),
            Operation::DirectoryChange(DirectoryChange {
                destination: "a".to_owned(),
            }),
            Operation::LsCommand(LsCommand {}),
            Operation::File(File {
                name: "e.txt".to_owned(),
                size: 42,
            }),
            Operation::DirectoryChange(DirectoryChange {
                destination: "e".to_owned(),
            }),
            Operation::LsCommand(LsCommand {}),
            Operation::File(File {
                name: "i.txt".to_owned(),
                size: 5,
            }),
        ];

        let result = to_file_tree(parsed);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_directory_sizes() {
        let a = File {
            name: "/var/a.txt".to_owned(),
            size: 10,
        };
        let b = File {
            name: "/var/b.txt".to_owned(),
            size: 11,
        };
        let c = File {
            name: "/usr/c.txt".to_owned(),
            size: 42,
        };
        let d = File {
            name: "/usr/local/d.txt".to_owned(),
            size: 11,
        };

        let tree: Vec<Event> = vec![Event::DirectoryListing(DirectoryListing {
            contents: vec![a, b, c, d],
        })];
        let result = dir_size(tree);

        let mut expected = vec![
            DirectoryTotal::new("/var", 21),
            DirectoryTotal::new("/", 74),
            DirectoryTotal::new("/usr", 53),
            DirectoryTotal::new("/usr/local", 11),
        ];

        assert_eq!(result, expected);
    }
}

#[derive(Debug, PartialEq)]
// stdout output
enum Operation {
    DirectoryChange(DirectoryChange),
    LsCommand(LsCommand),
    File(File),
    Directory(Directory),
}

// logical event of what happened
#[derive(Debug, PartialEq)]
enum Event {
    DirectoryChange(DirectoryChange),
    DirectoryListing(DirectoryListing),
    LsCommand(LsCommand),
}

#[derive(Debug, PartialEq)]
struct DirectoryChange {
    destination: String,
}

#[derive(Debug, PartialEq)]
struct LsCommand {}

#[derive(Debug, PartialEq, Clone)]
struct DirectoryListing {
    contents: Vec<File>,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct DirectoryTotal {
    name: String,
    size: usize,
}

impl DirectoryTotal {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Directory {
    name: String,
}

// if it begins with a $, it's a command
// if the command is ls, set state machine is_listing on
// then when you run into a command again, flip is_listing off
// if in directory listing, parse each line as a file, ignore dirs

// a horrible state machine
// let mut in_listing = false;
// in_listing = false;

// 🐶 no state, only parse, might be tempting to start doing state machine stuff
fn parse(input: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];

    let cd_regex = Regex::new(r"^\$\scd\s(.+)").unwrap();
    let ls_regex = Regex::new(r"^\$\sls").unwrap();
    let is_dir = Regex::new(r"^dir\s(.+)").unwrap();

    let mut listing: DirectoryListing = DirectoryListing { contents: vec![] };

    for line in input.lines() {
        let result: Operation = match line {
            s if cd_regex.is_match(s) => {
                let split: Vec<&str> = s.split_whitespace().collect();
                let destination = split[2].to_owned();
                Operation::DirectoryChange(DirectoryChange { destination })
            }
            s if ls_regex.is_match(s) => {
                listing.contents = vec![];
                Operation::LsCommand(LsCommand {})
            }
            s => {
                if !is_dir.is_match(s) {
                    let file_split: Vec<&str> = s.split_whitespace().collect();
                    let file = File {
                        size: file_split[0].parse().unwrap(),
                        name: file_split[1].to_owned(),
                    };
                    Operation::File(file)
                } else {
                    let split: Vec<&str> = s.split_whitespace().collect();
                    let dir = Directory {
                        name: split[1].to_owned(),
                    };
                    Operation::Directory(dir)
                }
            }
        };

        operations.push(result);
    }

    operations
}

// a file tree that is made up of under 100k files
fn to_file_tree(parsed: Vec<Operation>) -> Vec<Event> {
    let mut events: Vec<Event> = vec![];

    let mut pwd = "<unknown>".to_owned();
    let mut listing: DirectoryListing = DirectoryListing { contents: vec![] };
    let mut in_listing = false;

    for stdout_item in parsed {
        match stdout_item {
            Operation::DirectoryChange(e) => {
                let old_pwd = PathBuf::from(pwd.clone());

                // handle .. and / with path
                let destination = e.destination.to_owned();
                if destination == "/" {
                    pwd = "/".to_owned();
                }

                pwd = match destination.as_str() {
                    "/" => "/".to_owned(),
                    ".." => {
                        let path = Path::new(pwd.as_str());
                        let path = match path.parent() {
                            None => "/".to_owned(),
                            Some(e) => e.display().to_string(),
                        };
                        path
                    }
                    _ => Path::join(old_pwd.as_path(), e.destination.clone())
                        .display()
                        .to_string(),
                };

                in_listing = false;
                if !listing.contents.is_empty() {
                    events.push(Event::DirectoryListing(listing));
                    listing = DirectoryListing { contents: vec![] };
                }
            }
            Operation::LsCommand(e) => {
                in_listing = true;
            }
            Operation::File(e) => {
                let path = Path::new("");
                let full = path.join(&pwd).join(e.clone().name);
                if in_listing {
                    listing.contents.push(File {
                        name: full.display().to_string(),
                        size: e.size,
                    });
                }
            }
            Operation::Directory(e) => {}
            _ => println!("huh"),
        }
    }

    if in_listing {
        if !listing.contents.is_empty() {
            events.push(Event::DirectoryListing(listing));
            listing = DirectoryListing { contents: vec![] };
        }
        in_listing = false;
    }

    events
}

fn dir_size(tree: Vec<Event>) -> Vec<DirectoryTotal> {
    let mut totals: Vec<DirectoryTotal> = vec![];

    for item in tree {
        if let Event::DirectoryListing(e) = item {
            for file in e.contents {
                let mut at_root = false;
                let mut path = Path::new(&file.name);

                let mut current_path = path;
                while let Some(parent) = current_path.parent() {
                    let index = totals
                        .iter()
                        .position(|d| d.name == parent.display().to_string());

                    match index {
                        Some(i) => {
                            if let Some(dir) = totals.get_mut(i) {
                                dir.size += file.size;
                            } else {
                                panic!("Could not borrow dir as mutable?")
                            }
                        }
                        None => {
                            totals.push(DirectoryTotal {
                                name: parent.display().to_string(),
                                size: file.size,
                            });
                        }
                    }

                    current_path = parent;
                }
            }
        }
    }

    totals
}
