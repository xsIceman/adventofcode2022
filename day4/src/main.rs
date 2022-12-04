use std::fs;



fn main() {
      let input = fs::read_to_string(r"C:\Projects\adventOfCode2022\day4\input.txt").expect("read file");

//     let input = String::from(
//         r"2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8");
     step1(&input);
    step2(&input);
}

fn step1(input: &String) {
    let mut count = 0;
    let rows = input.split('\n');
    for row in rows {
        println!("row {row} ");
        let index_of_split = row.find(',').unwrap();
        let elf1 = &row[..index_of_split];
        let elf2 = &row[index_of_split+1..];

        let elf1_num = parse_num(elf1);
        let elf2_num = parse_num(elf2);

        if elf1_num.0 <= elf2_num.0 && elf1_num.1 >= elf2_num.1 {
            let x1 = elf1_num.0;
            let y1 = elf1_num.1;
            let x2 = elf2_num.0;
            let y2 = elf2_num.1;
            println!("e1 {x1} e1 {y1} e2 {x2} e2 {y2}");
            count += 1;
        }
        else if elf1_num.0 >= elf2_num.0 && elf1_num.1 <= elf2_num.1 {
            let x1 = elf1_num.0;
            let y1 = elf1_num.1;
            let x2 = elf2_num.0;
            let y2 = elf2_num.1;
            println!("e1 {x1} e1 {y1} e2 {x2} e2 {y2}");
            count += 1;
        }
        println!("");
    }
    println!("step1 {count}");
}

fn step2(input: &String) {
    let mut count = 0;
    let rows = input.split('\n');
    for row in rows {
        let index_of_split = row.find(',').unwrap();
        let elf1 = &row[..index_of_split];
        let elf2 = &row[index_of_split+1..];

        let elf1_num = parse_num(elf1);
        let elf2_num = parse_num(elf2);

        if elf1_num.0 >= elf2_num.0 && elf1_num.0 <= elf2_num.1 {
            count += 1;
        }
        else if elf1_num.1 >= elf2_num.0 && elf1_num.1 <= elf2_num.1 {
            count += 1;
        }
        else if elf2_num.0 >= elf1_num.0 && elf2_num.0 <= elf1_num.1 {
            count += 1;
        }
        else if elf2_num.1 >= elf1_num.0 && elf2_num.1 <= elf1_num.1 {
            count += 1;
        }
    }
    println!("step2 {count}");
}

fn parse_num(input: &str) -> (i32, i32) {
    let index_of_split = input.find('-').unwrap();
    let num1s = &input[..index_of_split];

    let num2s = &input[index_of_split+1..];
    // println!("num1 {num1s} num2 {num2s}");
    (num1s.parse().unwrap(), num2s.parse().unwrap())
}
