impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // map_or is faster then unwrap
        s.split_ascii_whitespace().last().map_or(0, |w| w.len()) as i32
    }
}
