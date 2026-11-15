use std::{
    collections::HashMap,
    fs::{self, TryLockError::WouldBlock},
};

fn main() {
    let input = fs::read_to_string("./input").unwrap();

    let groups = create_group(input);

    let sum = find_badge(groups);
    println!("{}", sum);
}
fn find_badge(groups: HashMap<i32, Vec<String>>) -> u32 {
    let mut sum = 0;
    let mut f = false;
    for (_i, group) in groups.into_iter() {
        for (i, line) in group.iter().enumerate() {
            if i == 0 {
                let letters = line.chars();
                if f == false {
                    letters.for_each(|c| {
                        if f == false {
                            if find_duplicate(&c, &group, Vec::new()) {
                                f = true;
                                sum += char_to_num(&c).unwrap();
                            }
                        }
                    });
                }
            }
        }
        f = false;
    }

    sum
}

fn find_duplicate(needle: &char, group: &Vec<String>, mut duplicate: Vec<char>) -> bool {
    let mut iter = group.iter();
    iter.next();
    let second_line = iter.next().unwrap();
    let third_line = iter.next().unwrap();

    if second_line.contains(needle.to_owned()) {
        if third_line.contains(needle.to_owned()) {
            return true;
        }
    }
    false
}
fn char_to_num(c: &char) -> Option<u32> {
    match c {
        'a'..='z' => Some((*c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Some((*c as u32) - ('A' as u32) + 27),
        _ => Some(0),
    }
}
fn create_group(input: String) -> HashMap<i32, Vec<String>> {
    let mut group_index = 0;
    let mut group = Vec::new();
    let mut groups = HashMap::new();

    for line in input.lines() {
        group.push(line.to_string());
        if group.len() == 3 {
            groups.insert(group_index, std::mem::take(&mut group));
            group_index += 1;
            group.clear();
        }
    }
    groups
}

#[test]
fn test_char_to_num() {
    let o = char_to_num(&'Z');
    assert_eq!(o, Some(52));
}

#[test]
fn get_needle() {
    let input = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    let groups = create_group(input);
    let f = find_badge(groups);
    assert_eq!(70, f);
}
#[test]
fn test_create_group() {
    let input = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    assert_eq!(create_group(input).len(), 2);
}
#[test]
fn test_find_duplicate() {
    let needle = 'Z';
    let group = vec![
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned(),
        "ttgJtRGJQctTZtZT".to_owned(),
        "CrZsJsPPZsGzwwsLwLmpwMDw".to_owned(),
    ];
    assert_eq!(find_duplicate(&needle, &group, Vec::new()), true);
}
