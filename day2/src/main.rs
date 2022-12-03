use std::fs;

fn main() {
    let file_content =
        fs::read_to_string(r"C:\Projects\adventOfCode2022\day2\input.txt").expect("file is read");
    // let file_content = String::from(r"A Y
    // B X
    // C Z");

    part2(file_content);
}

fn part2(file_content: String) {
    let rows = file_content.split('\n');

    let mut sum = 0;
    for row in rows {
        let mut cols = row.split(' ');
        let elf_play = cols.next().expect("data");
        let you_play = cols.next().expect("data");

        let mut youplay_val = 0;
        let mut elfplay_val = 0;
        match you_play {
            "Y" => youplay_val = 3,
            "X" => youplay_val = 0,
            "Z" => youplay_val = 6,
            _ => println!("none"),
        }

        match elf_play {
            "A" => elfplay_val = 1,
            "B" => elfplay_val = 2,
            "C" => elfplay_val = 3,
            _ => println!("none"),
        }
        sum = sum + find_result(elfplay_val, youplay_val)
    }
    println!("{sum}");
}

fn find_result(elf_play: i32, youplay: i32) -> i32 {
    if youplay == 3 {
        return 3 + elf_play; //draw
    }
    if youplay == 0 {
        //you loose
        if (elf_play - 1) == 0 {
            return 3;
        }
        return elf_play - 1;
    }
    if youplay == 6 {
        //you win
        if (elf_play + 1) == 4 {
            return 6 + 1;
        }
        return 6 + 1 + elf_play;
    }
    return 0;
}

fn part1(file_content: String) {
    let rows = file_content.split('\n');

    let mut sum = 0;
    for row in rows {
        let mut cols = row.split(' ');
        let elf_play = cols.next().expect("data");
        let you_play = cols.next().expect("data");

        let mut youplay_val = 0;
        let mut elfplay_val = 0;
        match you_play {
            "Y" => youplay_val = 2,
            "X" => youplay_val = 1,
            "Z" => youplay_val = 3,
            _ => println!("none"),
        }

        match elf_play {
            "A" => elfplay_val = 1,
            "B" => elfplay_val = 2,
            "C" => elfplay_val = 3,
            _ => println!("none"),
        }
        sum = sum + youplay_val + check_result(elfplay_val, youplay_val)
    }
    println!("{sum}");
}
//X, A Rock
// Y, B paper
// Z, C scissors
fn check_result(elf_play: i32, youplay: i32) -> i32 {
    if elf_play == youplay {
        return 3;
    }

    if elf_play > youplay {
        if elf_play == 3 && youplay == 1 {
            //elf play scissors you play rock
            return 6;
        }
        return 0;
    }
    if elf_play == 1 && youplay == 3 {
        //elf play rock you play scissors
        return 0;
    }
    return 6;
}
