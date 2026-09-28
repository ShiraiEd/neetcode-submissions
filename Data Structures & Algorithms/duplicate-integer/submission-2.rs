use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        for i in nums{
            if !set.insert(i){
                return true;
            }
        }
        false
    }
}
