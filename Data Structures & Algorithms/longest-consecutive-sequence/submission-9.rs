use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let seen: HashSet<i32> = nums.into_iter().collect();
        let mut res = 0;
        for i in &seen{
            if seen.contains(&(i-1)){
                continue;
            }
            let mut greater = i + 1;
            let mut count = 1;
            while seen.contains(&greater){
                count += 1;
                greater += 1;
            }
            res = res.max(count);
        }
        res
    }
}
