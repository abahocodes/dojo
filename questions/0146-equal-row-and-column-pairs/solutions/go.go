package main

import "fmt"

func equalPairs(grid [][]int) int {
	n := len(grid)
	rows := map[string]int{}
	for _, row := range grid {
		rows[fmt.Sprint(row)]++
	}
	pairs := 0
	col := make([]int, n)
	for c := 0; c < n; c++ {
		for r := 0; r < n; r++ {
			col[r] = grid[r][c]
		}
		pairs += rows[fmt.Sprint(col)]
	}
	return pairs
}
