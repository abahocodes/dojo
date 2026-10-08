package main

func orangesRotting(grid [][]int) int {
	rows, cols := len(grid), len(grid[0])
	state := make([][]int, rows) // don't mutate the caller's grid
	for r := range grid {
		state[r] = append([]int(nil), grid[r]...)
	}
	var frontier [][2]int
	fresh := 0
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if state[r][c] == 2 {
				frontier = append(frontier, [2]int{r, c})
			} else if state[r][c] == 1 {
				fresh++
			}
		}
	}

	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	minutes := 0
	for len(frontier) > 0 && fresh > 0 {
		minutes++
		var next [][2]int
		for _, cell := range frontier {
			for _, d := range dirs {
				nr, nc := cell[0]+d[0], cell[1]+d[1]
				if nr >= 0 && nr < rows && nc >= 0 && nc < cols && state[nr][nc] == 1 {
					state[nr][nc] = 2
					fresh--
					next = append(next, [2]int{nr, nc})
				}
			}
		}
		frontier = next
	}
	if fresh != 0 {
		return -1
	}
	return minutes
}
