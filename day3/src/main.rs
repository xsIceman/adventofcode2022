use std::collections::HashMap;
use std::fs;


fn main() {
    let input = fs::read_to_string(r"C:\Projects\adventOfCode2022\day3\input.txt").expect("read file");

//     let input = String::from(
//         r"vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw",
//     );
    step1(&input);
    step2(&input);
}

fn step2(input: &String) {
    let rows = input.split('\n');
    let mut priority = Vec::new();
    let mut group = 0;
    let mut count = HashMap::new();

    for row in rows {
        println!("{row}");

        let mut letters = HashMap::new();
        for c in row.chars(){
            letters.insert(c, 1);
        }
        
        for c in letters.keys() {
            count.entry(*c).and_modify(|f| *f += 1).or_insert(1);
        }
        
        {
            let ccount = &mut count;
            group = group + 1;
            if group == 3 {
                group = 0;
                for entry in ccount {
                    if *entry.1 == 3 {
                        
                        let pri = *entry.0 as u32;
                        priority.push(pri);
                        println!("{pri}");
                    }
                }
                count.clear();
                println!("");
                println!("");
            }
        }
    }
    //65 A, Z 90
    //97 a, z 122

    //a -z 1 26
    //A - Z 27 52
    let mut sum_pri = 0;
    for p in priority {
        println!("{p} ");
        if p > 90 {
            sum_pri = sum_pri + p - 96;
        } else {
            sum_pri = sum_pri + p - 38;
        }
    }

    println!("total sum {sum_pri}");
}

fn step1(input: &String) {
    let rows = input.split('\n');
    let mut priority = Vec::new();
    for row in rows {
        let mut part1_count = HashMap::new();

        // let mut part2_count = HashMap::new();

        let len = row.len();
        let half = len / 2;

        // println!("len {len} half {half}");
        let half_byte = row.chars().map(|c| c.len_utf8()).take(half).sum();

        // println!("half_byte {half_byte} ");

        let part1 = &row[..half_byte];
        let part2 = &row[half_byte..];
        // println!(" {row}");
        // println!(" {part1}  {part2}\n");

        for c in part1.chars() {
            if !part1_count.contains_key(&c) {
                part1_count.insert(c, 1);
            };
        }

        for c in part2.chars() {
            if part1_count.contains_key(&c) {
                let cuint = c as u32;
                priority.push(cuint);
                println!("{c} {cuint}");
                break;
            }
        }
    }
    //65 A, Z 90
    //97 a, z 122

    //a -z 1 26
    //A - Z 27 52
    let mut sum_pri = 0;
    for p in priority {
        println!("{p} ");
        if p > 90 {
            sum_pri = sum_pri + p - 96;
        } else {
            sum_pri = sum_pri + p - 38;
        }
    }

    println!("total sum {sum_pri}");
}

// for c in part2.chars() {
//     if part2_count.contains_key(&c) {
//         part2_count.insert(c, 2);
//     } else {
//         part2_count.insert(c, 1);
//     };
// }

// for item in part1_count {
//     if item.1 > 1 {
//         let output = item.0;
//         println!("{output}");
//     }
// }

// for item in part2_count {
//     if item.1 > 1 {
//         let output = item.0;
//         println!("{output}");
//     }
// }
