impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        if strs.len() == 0{
            return "".to_string()
        }
        let mut output = String::from("");
        
        for s in &strs{
            let c = format!("{:03}{}", s.len(), s);
            output.push_str(&c);
        }
        println!("{output}");
        output
    }

    pub fn decode(s: String) -> Vec<String> {
        if s.len() == 0{
            return vec![];
        }
        let mut res = Vec::<String>::new();        
        let mut count = 0;
        while count < s.len(){
            let l: usize = s[count..count+3].parse().unwrap();
            let word_start = count+3;
            let word_stop = l + count + 3;
            res.push(s[word_start..word_stop].to_string());
            count = word_stop;
        }

        res
    }
}
