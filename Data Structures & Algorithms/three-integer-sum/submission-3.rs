impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        for a in 0..nums.len() {
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }
            let mut b = a + 1;
            let mut c = nums.len().wrapping_sub(1);
            while b < c {
                let sum = nums[a] + nums[b] + nums[c];
                if sum == 0 {
                    result.push(vec![nums[a], nums[b], nums[c]]);
                    b += 1;
                    while b < c && nums[b] == nums[b - 1] {
                        b += 1;
                    }
                    c -= 1;
                    while b < c && nums[c] == nums[c + 1] {
                        c -= 1;
                    }
                } else if sum < 0 {
                    b += 1;
                } else {
                    c -= 1;
                }
            }
        }
        result
    }
}