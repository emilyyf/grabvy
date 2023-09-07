use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

fn get_rolls() -> [u64; 5] {
    rand::thread_rng().gen::<[u64; 5]>().map(|a| (a % 6))
}

fn convert_rolls_to_line(rolls: [u64; 5]) -> u64 {
    let mut a: u64 = 0;
    for (i, roll) in rolls.iter().enumerate() {
        a += roll * 6_u64.pow((4 - i) as u32);
    }

    a
}

fn main() {
    for _ in 0..6 {
        let rolls = get_rolls();
        let line = convert_rolls_to_line(rolls);

        let file = File::open("wordlist.txt").expect("Failed to open wordlist file");
        let mut lines = io::BufReader::new(file).lines();
        let a = lines
            .nth(line as usize)
            .expect("Unable to get line")
            .unwrap();

        println!("{}", a);
    }
}
