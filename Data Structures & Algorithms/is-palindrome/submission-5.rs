impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        if b.is_empty() {
            return true
        }
 
        let (mut left, mut right) = (0, b.len() - 1);

        while left < right{
            if !b[left].is_ascii_alphanumeric(){
                left += 1;
            } else if !b[right].is_ascii_alphanumeric(){
                right -= 1;
            } else {
                if b[left].to_ascii_lowercase() != b[right].to_ascii_lowercase(){
                    return false;
                }
                left += 1;
                right -= 1;
            }
        }
        true
    }
}
