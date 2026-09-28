use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut freq = HashMap::<i32, i32>::new();
        for &n in &nums{
            *freq.entry(n).or_insert(0) += 1;
        }

        let mut res = Vec::<i32>::with_capacity(k as usize);
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
        
        for (&num, &f) in &freq{
            buckets[f as usize].push(num);
        }

        for b in (1..=nums.len()).rev(){
            for &n in &buckets[b]{
                res.push(n);
                if res.len() == k as usize{
                    return res;
                }
            }
        }

        res
    }
}
