package main

import "sort"

func longestIncreasingPath(matrix [][]int) int {
	rows, cols := len(matrix), len(matrix[0])
	// cells by decreasing height, encoded as r*cols + c
	order := make([]int, rows*cols)
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool {
		x, y := order[a], order[b]
		return matrix[x/cols][x%cols] > matrix[y/cols][y%cols]
	})

	best := make([][]int, rows)
	for r := range best {
		best[r] = make([]int, cols)
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	answer := 1
	for _, cell := range order {
		r, c := cell/cols, cell%cols
		height := matrix[r][c]
		length := 1
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr >= 0 && nr < rows && nc >= 0 && nc < cols && matrix[nr][nc] > height {
				length = max(length, best[nr][nc]+1)
			}
		}
		best[r][c] = length
		answer = max(answer, length)
	}
	return answer
}
