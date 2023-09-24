use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

fn convert_rolls_to_line(rolls: [u64; 5]) -> u64 {
    let mut a: u64 = 0;
    for (i, roll) in rolls.iter().enumerate() {
        a += roll * 6_u64.pow((4 - i) as u32);
    }

    a
}

fn main() {
    let mut words: Vec<String> = Vec::new();
    for _ in 0..6 {
        let rolls = rand::thread_rng().gen::<[u64; 5]>().map(|a| (a % 6));
        let line = convert_rolls_to_line(rolls);

        let file = File::open("wordlist.txt").expect("Failed to open wordlist file");
        let mut lines = io::BufReader::new(file).lines();
        let word = lines
            .nth(line as usize)
            .expect("Unable to get line")
            .unwrap();

        words.push(word);
    }

    println!("{:?}", words)
}
