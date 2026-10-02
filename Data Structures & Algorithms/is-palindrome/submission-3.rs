impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let char_list: Vec<char> = 
        s.chars()
        .filter(|&x| x.is_ascii_alphanumeric())
        .map(|x| x.to_ascii_lowercase())
        .collect();

        let len = char_list.len();

        if len <= 1{
            return true;
        }
        let mut left = 0;
        let mut right = len - 1;

        while left <= right{
            if char_list[left] != char_list[right]{
                return false;
            }
            left += 1;
            right -= 1;
        } 
        
        true
    }
}
