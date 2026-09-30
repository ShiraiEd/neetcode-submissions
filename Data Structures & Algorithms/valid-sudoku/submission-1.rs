use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut column_m: HashMap<char, Vec<usize>> = HashMap::new();
        let mut box_m: HashMap<usize, Vec<char>> = HashMap::new();
        for (row_idx, row) in board.iter().enumerate(){
            let mut line_m: HashSet<char> = HashSet::new();
            for (col, &ch) in row.iter().enumerate(){
                if ch != '.'{
                    if !line_m.insert(ch){
                        return false;
                    }
                    let cols = column_m.entry(ch).or_default();
                    if cols.contains(&col){
                        return false;
                    }
                    cols.push(col);

                    let block = box_m.entry((row_idx / 3) * 3 + (col / 3)).or_default();
                    if block.contains(&ch){
                        return false;
                    }
                    block.push(ch);
                }

            }
        }
        true
    }
}
