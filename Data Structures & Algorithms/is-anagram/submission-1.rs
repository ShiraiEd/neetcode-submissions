impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let sum_s: u8 = s.into_bytes().iter().map(|x| x*x).sum();
        let sum_t: u8 = t.into_bytes().iter().map(|x| x*x).sum();
        if sum_s == sum_t{
            return true
        } else {
            return false
        }
    }
}
