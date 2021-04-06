use std::collections::HashMap;
fn main() {
    println!("{}", length_of_longest_substring(String::from("tmmzuxt")));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0usize;
    let mut length = 0usize;
    let mut i = 0usize;
    let sl = s.len();
    let mut m :HashMap<u8, usize> = HashMap::new();
    let chars = s.as_bytes();
    while i < sl {
        match m.get(&chars[i]) {
            Some(l) => {
                if l >= &(i - length) {
                    length = i - l;
                } else {
                    length = length + 1
                }
            }
            None => {
                length = length + 1
            }
        }
        m.insert(chars[i], i);
        max_len = max(max_len, length);
        i = i + 1;
    }
    max_len as i32
}

pub fn max(i: usize, j: usize) -> usize {
    if i > j {
        return i
    } else {
        return j;
    }
}
