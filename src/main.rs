use std::{env, fs::read};

fn main() {
    let path = env::args().nth(1).expect("no path passed in");
    dbg!(&path);
    let file = read(path).expect("file doesn't exist");
    let amount_of_bytes = file.len();
    let amount_of_ascii = file.iter().fold(0, |acc,byte|{acc + byte.is_ascii() as usize});
    dbg!(amount_of_ascii);
    dbg!(amount_of_bytes);
    println!("{}",amount_of_ascii as f64 /amount_of_bytes as f64);
}
