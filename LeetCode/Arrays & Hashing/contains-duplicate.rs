use std::collections::HashMap;

impl Solution {
    pub
 
fn
 
contains_duplicate(nums: Vec<i32>) -> bool {
        let
 
mut map: HashMap<i32, bool> = HashMap::new();
        let mut result: bool = false;

        for num in nums {
            if map.contains_key(&num) {
                result = true;
                break;
            } else {
                map.insert(num, true);
            }
        }

        return result;
    }
}