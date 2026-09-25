use std::collections::HashMap;
use std::collections::hash_map::Entry;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut reg: HashMap<i32, usize> = HashMap::new();
        for (i, v) in nums.iter().enumerate(){
            let mis = target - v;
            match reg.entry(mis) {
                Entry::Occupied(m) => {
                    return vec![*m.get() as i32, i as i32]
                }
                Entry::Vacant(_) => {
                }
            }
            reg.insert(*v as i32, i);

        }
        Vec::<i32>::new()

    }
}
