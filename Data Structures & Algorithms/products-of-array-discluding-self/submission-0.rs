
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res = vec![1i32; len];
        let mut prefix = 1;
        for i in 0..len{
            res[i] *= prefix;
            prefix *= nums[i];
        }
        let mut postfix = 1;
        for i in (0..len).rev(){
            res[i] *= postfix;
            postfix *= nums[i];
        }
        res
    }
}

