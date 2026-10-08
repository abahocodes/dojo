package main

func numIslands(grid [][]string) int {
	if len(grid) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	seen := make([][]bool, rows)
	for r := range seen {
		seen[r] = make([]bool, cols)
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	count := 0
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if grid[r][c] != "1" || seen[r][c] {
				continue
			}
			count++
			seen[r][c] = true
			stack := [][2]int{{r, c}}
			for len(stack) > 0 {
				cell := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				for _, d := range dirs {
					ni, nj := cell[0]+d[0], cell[1]+d[1]
					if ni >= 0 && ni < rows && nj >= 0 && nj < cols && grid[ni][nj] == "1" && !seen[ni][nj] {
						seen[ni][nj] = true
						stack = append(stack, [2]int{ni, nj})
					}
				}
			}
		}
	}
	return count
}
