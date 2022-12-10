use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string(r"C:\Projects\adventOfCode2022\day7\input.txt").expect("read file");
//     let input = r"$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k";

    step2(&input);
}

fn step1(input: &str) {
    let lines = input.lines();

    let mut current_path: Vec<String> = Vec::new();
    let mut dirs = HashMap::new();
    let mut dir_size = 0;
    let mut store_val = true;

    for line in lines {
        if line.starts_with('$') {
            let mut lineParts = line.split_whitespace();
            match lineParts.nth(1) {
                Some("cd") => {
                    if store_val {
                        let path = join_path(&current_path);
                        dirs.insert(path, dir_size);
                        dir_size = 0;
                        store_val = false;
                    }

                    let dirName = lineParts.next().unwrap();
                    if dirName == "/" {
                        current_path.clear();
                    } else if dirName == ".." {
                        current_path.pop();
                    } else {
                        let p_to_push = format!("/{}", dirName);
                        current_path.push(p_to_push);
                    }
                }
                Some("ls") => {
                    store_val = true;
                }
                _ => {
                    println!("other");
                }
            }
        } else if line.starts_with("dir") {
        } else {
            let mut lineParts = line.split_whitespace();
            let size = lineParts.next().unwrap().parse::<i32>().unwrap();
            dir_size += size;
        }
    }
    if store_val {
        let path = join_path(&current_path);
        dirs.insert(path, dir_size);
        dir_size = 0;
        store_val = false;
    }
    let mut sum_less_than_100000 = 0;
    let mut count_vec:Vec<(&String, &i32)> = dirs.iter().collect();
    count_vec.sort_by(|a, b| b.0.split('/').count().cmp(&a.0.split('/').count()));
    for (path, size) in count_vec {
        // println!("path {path}");
        let mut folder_size = 0;
        for (f, s) in dirs.iter() {
            // println!("f {f}");
            if f.starts_with(path)  {
                // println!("starts_with {f} s {s}");
                folder_size += s;
            }
           
        }
        if folder_size < 100000 {
            sum_less_than_100000 += folder_size;
        }
    }

    println!(" sum_less_than_100000 {sum_less_than_100000} ");

    //update all folders with size of subfolders
    // {
    //     let r_dirs = dirs;
    //     let mut sum_less_than_100000 = 0;
    //     for (f, s) in r_dirs {
    //         println!(" folder {f} size {s}");
    //         if s < 100000 {
    //             sum_less_than_100000 += s;
    //         }
    //     }
    //     println!(" sum_less_than_100000 {sum_less_than_100000} ");
    // }
    
}


fn step2(input: &str) {
    let lines = input.lines();

    let mut current_path: Vec<String> = Vec::new();
    let mut dirs = HashMap::new();
    let mut dir_size = 0;
    let mut store_val = true;

    for line in lines {
        if line.starts_with('$') {
            let mut lineParts = line.split_whitespace();
            match lineParts.nth(1) {
                Some("cd") => {
                    if store_val {
                        let path = join_path(&current_path);
                        dirs.insert(path, dir_size);
                        dir_size = 0;
                        store_val = false;
                    }

                    let dirName = lineParts.next().unwrap();
                    if dirName == "/" {
                        current_path.clear();
                    } else if dirName == ".." {
                        current_path.pop();
                    } else {
                        let p_to_push = format!("/{}", dirName);
                        current_path.push(p_to_push);
                    }
                }
                Some("ls") => {
                    store_val = true;
                }
                _ => {
                    println!("other");
                }
            }
        } else if line.starts_with("dir") {
        } else {
            let mut lineParts = line.split_whitespace();
            let size = lineParts.next().unwrap().parse::<i32>().unwrap();
            dir_size += size;
        }
    }
    if store_val {
        let path = join_path(&current_path);
        dirs.insert(path, dir_size);
        dir_size = 0;
        store_val = false;
    }
    let mut small_to_delete = 999999999;
    let mut count_vec:Vec<(&String, &i32)> = dirs.iter().collect();
    count_vec.sort_by(|a, b| b.0.split('/').count().cmp(&a.0.split('/').count()));

    let mut total_disk = 0;

    for (_, size) in &dirs {
        total_disk+=size;
    }

    let disk_needed =  30000000 - ( 70000000 - total_disk);
    println!(   "total_disk {total_disk} disk_needed {disk_needed} ");

    for (path, size) in count_vec {
        // println!("path {path}");
        let mut folder_size = 0;
        for (f, s) in dirs.iter() {
            // println!("f {f}");
            if f.starts_with(path)  {
                // println!("starts_with {f} s {s}");
                folder_size += s;
            }
           
        }
        println!(" path {path} size {folder_size} ");
        if folder_size >= disk_needed {
            
            if small_to_delete > folder_size
            {
                small_to_delete = folder_size;
            }
        }
    }

    println!(" small_to_delete {small_to_delete} ");

    //update all folders with size of subfolders
    // {
    //     let r_dirs = dirs;
    //     let mut sum_less_than_100000 = 0;
    //     for (f, s) in r_dirs {
    //         println!(" folder {f} size {s}");
    //         if s < 100000 {
    //             sum_less_than_100000 += s;
    //         }
    //     }
    //     println!(" sum_less_than_100000 {sum_less_than_100000} ");
    // }
    
}

//take all items in vec and add them together in string
fn join_path(path: &Vec<String>) -> String {
    let mut result = String::new();
    for item in path {
        result.push_str(item);
    }
    result
}
