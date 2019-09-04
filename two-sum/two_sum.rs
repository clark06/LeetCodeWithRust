use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashNums = HashMap::new();
        let mut index = 0;
        for num in nums {
            if let Some(indexLeft) = hashNums.get(&(target - num)) {
                return vec![*indexLeft as i32, index as i32];
            }
            hashNums.insert(num, index);
            index += 1;
        }
        return vec![-1, -1];
    }
}