use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut ch_columns = vec![HashSet::<char>::new(); 9];
        let mut ch_rows = vec![HashSet::<char>::new(); 9];
        let mut ch_boxes = vec![HashSet::<char>::new(); 9];
        for r in 0..9{
            for c in 0..9{
                let ch = board[r][c];
                if ch == '.'{
                    continue;
                }
                let b = (r / 3) * 3 + (c / 3);
                if !(ch_rows[r].insert(ch)) || !(ch_columns[c].insert(ch)) || !(ch_boxes[b].insert(ch)){
                    return false;
                } 
            }
        }
        true
    }
}
