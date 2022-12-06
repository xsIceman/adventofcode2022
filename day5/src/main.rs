use std::fs;

fn main() {
    let input = fs::read_to_string(r"C:\Projects\adventOfCode2022\day5\input.txt").expect("read file");
//     let input = r"    [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2";

    step1(&input);
}

fn step1(input: &str) {
    let mut stack_input = Vec::new();
    let mut operation_input = Vec::new();
    let mut is_operation = false;
    let lines = input.lines();

    //reverse stack input lines
    for l in lines {
        if l.is_empty() {
            is_operation = true;
        } else {
            if is_operation {
                operation_input.push(l);
            } else {
                stack_input.insert(0, l);
            }
        }
    }
    const num_of_stacks : usize = 9;
    let mut first_line = true;
    let mut array: [Vec<char>; num_of_stacks] = Default::default();
    for ss in stack_input {
        println!("{ss}");
        if first_line {
            first_line = false;
            //parse ss string for numbers
            //and push them to a vector
            let mut nums = Vec::new();
            for c in ss.chars() {
                if c.is_numeric() {
                    nums.push(c);
                }
            }
        } else {
            //parse ss for letters and push to stack

            //for loop that takes 4 chars at a time
            //and parses them for letters
            let mut i = 0;

            for c in ss.chars() {
                i += 1;
                if c.is_alphabetic() {
                    let c_stack_num = i / 4;
                    array[c_stack_num].push(c);
                    println!("letter: {c} on to stack {c_stack_num}");
                }
            }
        }
    }
    //iterate over operation_input and parse numbers from each line and run the operation
    for op in operation_input {
        let data_input: Vec<&str> = op.split_ascii_whitespace().collect();
        let num_move = data_input[1].parse::<usize>().unwrap();
        let num_from = data_input[3].parse::<usize>().unwrap();
        let num_to = data_input[5].parse::<usize>().unwrap();

        println!("moving {num_move} from {num_from} to {num_to}");
        for _n in 0..num_move {
            let item = array[num_from-1].pop().unwrap();
            array[num_to - 1].push(item);
        }
    }

    //print out top element of each stack
    for i in 0..num_of_stacks {
        let top_element = array[i].pop().unwrap();
        println!("stack {i} top element: {top_element}");
    }

}
