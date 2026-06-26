use std::{collections::HashMap, io};
    
const ABC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NOP: &str = "nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM";

fn main() {
    let mut map = HashMap::new();
    map.extend(ABC.chars().zip(NOP.chars()));

    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        for c in buf.chars() {
            print!("{}", map.get(&c).unwrap_or(&c));
        }
        buf.clear();
    }
}
