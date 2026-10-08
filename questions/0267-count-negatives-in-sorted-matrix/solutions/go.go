package main

func countNegatives(grid [][]int) int {
	n := len(grid[0])
	row, col, count := len(grid)-1, 0, 0
	for row >= 0 && col < n {
		if grid[row][col] < 0 {
			count += n - col
			row--
		} else {
			col++
		}
	}
	return count
}
