use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut reg: HashMap<i32, usize> = HashMap::new();
        for (i, v) in nums.iter().enumerate(){
            let mis = target - v;
            if reg.contains_key(&mis) {
                return vec![*reg.get(&mis).unwrap() as i32, i as i32];
            }
            reg.insert(*v as i32, i);
        }
        vec![]
    }
}
