impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }
        let mut hash: [i32; 26] = [0; 26];
        for a in s.bytes() {
            hash[(a - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            let idx = (b - b'a') as usize;
            hash[idx] -= 1;
            if hash[idx] < 0{
                return false
            }
        }
        hash.iter().all(|&x| x == 0)
    }
}
