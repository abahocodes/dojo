package main

func exist(board [][]string, word string) bool {
	rows, cols := len(board), len(board[0])
	grid := make([][]byte, rows)
	var onBoard [256]int
	for r := range board {
		grid[r] = make([]byte, cols)
		for c := range board[r] {
			grid[r][c] = board[r][c][0]
			onBoard[grid[r][c]]++
		}
	}
	w := []byte(word)
	var needed [256]int
	for _, ch := range w {
		needed[ch]++
		if needed[ch] > onBoard[ch] {
			return false
		}
	}
	// a path read backwards is still a path; start from the rarer end to prune sooner
	if onBoard[w[0]] > onBoard[w[len(w)-1]] {
		for i, j := 0, len(w)-1; i < j; i, j = i+1, j-1 {
			w[i], w[j] = w[j], w[i]
		}
	}

	var dfs func(r, c, i int) bool
	dfs = func(r, c, i int) bool {
		if r < 0 || r >= rows || c < 0 || c >= cols || grid[r][c] != w[i] {
			return false
		}
		if i == len(w)-1 {
			return true
		}
		grid[r][c] = '#' // mark as used on the current path
		found := dfs(r+1, c, i+1) || dfs(r-1, c, i+1) || dfs(r, c+1, i+1) || dfs(r, c-1, i+1)
		grid[r][c] = w[i]
		return found
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if dfs(r, c, 0) {
				return true
			}
		}
	}
	return false
}
