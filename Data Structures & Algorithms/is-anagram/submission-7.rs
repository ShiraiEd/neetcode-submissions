impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }
        let mut hash: [i32; 26] = [0; 26];
        for (a, b) in s.bytes().zip(t.bytes()) {
            hash[(a - b'a') as usize] += 1;
            hash[(b - b'a') as usize] -= 1;
        }
        hash.iter().all(|&x| x == 0)
    }
}
