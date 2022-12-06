use std::fs;
use std::hash::Hash;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string(r"C:\Projects\adventOfCode2022\day6\input.txt").expect("read file");
    // let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    step2(&input);
}

fn step1(input: &str) {
    let mut buffer = Vec::new();

    for (i, c) in input.chars().enumerate() {
        buffer.insert(0, c);
        if buffer.len() == 5{
            buffer.pop();
        }

        if buffer.len() == 4{
            if has_unique_elements(buffer.iter()){
                let i    = i +1;
                println!("unique: {i}");
                return;
            }
        }
    }
}

fn step2(input: &str) {
    let mut buffer = Vec::new();

    for (i, c) in input.chars().enumerate() {
        buffer.insert(0, c);
        if buffer.len() == 15{
            buffer.pop();
        }

        if buffer.len() == 14{
            if has_unique_elements(buffer.iter()){
                let i    = i +1;
                println!("unique: {i}");
                return;
            }
        }
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
