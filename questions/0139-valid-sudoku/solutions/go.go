package main

func isValidSudoku(board []string) bool {
	var rows, cols, boxes [9]int
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			ch := board[r][c]
			if ch == '.' {
				continue
			}
			bit := 1 << (ch - '1')
			b := (r/3)*3 + c/3
			if (rows[r]|cols[c]|boxes[b])&bit != 0 {
				return false
			}
			rows[r] |= bit
			cols[c] |= bit
			boxes[b] |= bit
		}
	}
	return true
}
