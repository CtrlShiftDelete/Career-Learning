impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort();
        
        let mut t_chars: Vec<char> = t.chars().collect();
        t_chars.sort();
        
        s_chars == t_chars
    }
}
