impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut ch_columns = [[false; 9]; 9];
        let mut ch_rows = [[false; 9]; 9];
        let mut ch_boxes = [[false; 9]; 9];
        for r in 0..9{
            for c in 0..9{
                let ch = match board[r][c]{
                    '.' => continue,
                    x => x as usize - '1' as usize,
                } ;
                let b = (r / 3) * 3 + (c / 3);
                if ch_rows[r][ch] || ch_columns[c][ch] || ch_boxes[b][ch]{
                    return false;
                }
                ch_rows[r][ch] = true;
                ch_columns[c][ch] = true;
                ch_boxes[b][ch] = true;
            }
        }
        true
    }
}
