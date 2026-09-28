impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut output = String::new();
        for s in &strs{
            let c = format!("{:03}{}", s.len(), s);
            output.push_str(&c);
        }
        output
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::<String>::new();        
        let mut count = 0;
        while count < s.len(){
            let l: usize = s[count..count+3].parse().unwrap();
            let word_start = count+3;
            let word_stop = word_start + l;
            res.push(s[word_start..word_stop].to_string());
            count = word_stop;
        }

        res
    }
}
