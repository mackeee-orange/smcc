pub fn check_kaibun(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if chars[chars.len() - (i + 1)] != *c {
            return false;
        }
    }
    true
}