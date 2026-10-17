package main

func maxAreaOfIsland(grid [][]int) int {
	rows := len(grid)
	cols := 0
	if rows > 0 {
		cols = len(grid[0])
	}
	seen := make([][]bool, rows)
	for i := range seen {
		seen[i] = make([]bool, cols)
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	best := 0
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if grid[r][c] != 1 || seen[r][c] {
				continue
			}
			// Iterative DFS: count every land cell reachable from (r, c).
			seen[r][c] = true
			stack := [][2]int{{r, c}}
			area := 0
			for len(stack) > 0 {
				cell := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				area++
				for _, d := range dirs {
					ny, nx := cell[0]+d[0], cell[1]+d[1]
					if ny >= 0 && ny < rows && nx >= 0 && nx < cols && grid[ny][nx] == 1 && !seen[ny][nx] {
						seen[ny][nx] = true
						stack = append(stack, [2]int{ny, nx})
					}
				}
			}
			if area > best {
				best = area
			}
		}
	}
	return best
}
