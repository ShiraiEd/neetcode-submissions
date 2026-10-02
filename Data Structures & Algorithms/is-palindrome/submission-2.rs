impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let lenght = s.len();
        let mut left = 0;
        let mut right = lenght - 1;
        let chars: Vec<char> = s.trim().to_lowercase().chars().collect();
        for _ in 0..(lenght/2){
            let mut lc = &chars[left];
            while !lc.is_alphanumeric() && left != right{
                left += 1;
                lc = &chars[left];
            }
            let mut rc = &chars[right];
            while !rc.is_alphanumeric() && right != left{
                right -= 1;
                rc = &chars[right];
            }
            println!("rc{right}: {rc} lc{left}: {lc}");
            if lc != rc{
                return false;
            }
            if right == left{
                return true;
            }
            left += 1;
            right -=1;
        }
        true
    }
}
