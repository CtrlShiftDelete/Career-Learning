use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map = HashMap::new();
        let mut result = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&prev_index) = map.get(&(target - num)) {
                result.push(prev_index as i32);
                result.push(i as i32);
                return result;
            }
            map.insert(num, i);
        }

        result
    }
}

