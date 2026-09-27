use std::collections::{HashMap, BTreeMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 1 && k == 1{
            return nums
        }

        let mut freq = HashMap::<i32, i32>::new();
        for n in nums{
            *freq.entry(n).or_insert(0) += 1;
        }

        let mut buckets = BTreeMap::<i32, Vec<i32>>::new();
        let mut res = Vec::<i32>::new();
        for (k, v) in freq{
            buckets.entry(v).or_default().push(k)
        }
        for (_, bucket) in buckets.iter().rev(){
            for &n in bucket{
                res.push(n);
                if res.len() == k as usize{
                    return res;
                }
            }
        }

        res
    }
}
