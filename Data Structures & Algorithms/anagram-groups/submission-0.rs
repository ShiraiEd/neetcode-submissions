use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() <= 1{
            return vec![strs];
        }
        let mut result: Vec<Vec<String>> = vec![];

        //this will need a hashmap with the sorted string as key
        //and the list of words as values
        //each iteration will get the word
        //clone into another var
        //sort the word and insert in the map
        //in the end, the lists from the map will be extracted into result

        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for (i, w) in strs.iter().enumerate(){
            let copy = w.clone();
            let mut s = w.chars().collect::<Vec<char>>();
            s.sort();
            let s: String = s.into_iter().collect();
            map.entry(s).or_default().push(copy);
        }

        for (sorted, words) in map{
            result.push(words);
        }

        result
    }
}
