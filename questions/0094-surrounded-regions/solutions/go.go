package main

func captureRegions(board [][]string) [][]string {
	rows, cols := len(board), len(board[0])
	safe := make([][]bool, rows)
	for r := range safe {
		safe[r] = make([]bool, cols)
	}
	var stack [][2]int
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			onEdge := r == 0 || c == 0 || r == rows-1 || c == cols-1
			if onEdge && board[r][c] == "O" {
				safe[r][c] = true
				stack = append(stack, [2]int{r, c})
			}
		}
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(stack) > 0 {
		cell := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, d := range dirs {
			nr, nc := cell[0]+d[0], cell[1]+d[1]
			if nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr][nc] == "O" && !safe[nr][nc] {
				safe[nr][nc] = true
				stack = append(stack, [2]int{nr, nc})
			}
		}
	}
	out := make([][]string, rows)
	for r := range out {
		out[r] = make([]string, cols)
		for c := range out[r] {
			if safe[r][c] {
				out[r][c] = "O"
			} else {
				out[r][c] = "X"
			}
		}
	}
	return out
}
