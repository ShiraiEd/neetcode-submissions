use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() <= 1{
            return vec![strs];
        }

        let mut map = HashMap::<[u8;26], Vec<String>>::new();

        for s in strs{
            let mut key = [0u8; 26];
            for b in s.as_bytes(){
                key[(b - b'a') as usize] += 1;
            }

            map.entry(key).or_default().push(s);

        }
        map.into_values().collect()    
    }
}
