use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut mp: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut sorted_chars: Vec<char> = s.chars().collect();
            sorted_chars.sort();

            let sorted_str: String = sorted_chars.iter().collect();

            let entry = mp.entry(sorted_str).or_insert(vec![]);
              entry.push(s.clone());
        }

        let res: Vec<Vec<String>> = mp.into_iter().map(|(_, v)| v).collect();

        res
    }
}
