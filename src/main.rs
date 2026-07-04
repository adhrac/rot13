use std::{collections::HashMap, io};
    

fn main() {
    let map = rot_n_character_map(13);

    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        for c in buf.chars() {
            print!("{}", map.get(&c).unwrap_or(&c));
        }
        buf.clear();
    }
}

// n > 0: moves characters right
// n < 0: moves characters left
fn rotate_by(s: &str, n: isize) -> impl Iterator<Item = char> {
    let n = -n;
    if n > 0 {
        s.chars().cycle().skip(n as usize).take(s.len())
    }
    else {
        let signed_len = s.len() as isize;
        s.chars().cycle().skip((signed_len + (n % signed_len)) as usize).take(s.len())
    }
}

fn rot_n_character_map(n: isize) -> HashMap<char, char> {
    static ABC: &str = "abcdefghijklmnopqrstuvwxyz";

    let mut map = HashMap::new();
    map.extend(ABC.chars().zip(rotate_by(ABC, n)));
    map.extend(ABC.to_ascii_uppercase().chars().zip(rotate_by(&ABC.to_ascii_uppercase(), n)));

    map
}