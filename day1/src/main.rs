use std::fs;


fn main() {
    println!(r"C:\Projects\adventOfCode2022\day1\input.txt");
let content = fs::read_to_string(r"C:\Projects\adventOfCode2022\day1\input.txt").expect("should have read file");
    let rows = content.split('\n');

    let mut v= Vec::new();

    let mut sum = 0;

    for r in rows {
        if r.is_empty()
        {
            println!("{sum}");
            v.push(sum);
            sum = 0;
        }
        else{
            let my_int: i32 = r.parse().unwrap();
            sum = sum + my_int;
        }
    }
    println!("max ");
    let max = v.iter().max();
    match max {
        Some(toprint)=> println!("{toprint}"),
        None => println!("no max")
    }

     v.sort();
     sum = 0;
for (i, r ) in v.iter().rev().enumerate() {
    sum = sum + r;
    if i == 2
    {break;}
}
println!("max {sum}");
    
}
