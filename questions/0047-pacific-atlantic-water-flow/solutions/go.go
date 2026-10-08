package main

func pacificAtlantic(heights [][]int) [][]int {
	rows, cols := len(heights), len(heights[0])
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

	// Walk uphill from the ocean: water can flow from each reached cell
	// down to the ocean. Iterative DFS keeps large grids off the call stack.
	reachable := func(starts [][2]int) [][]bool {
		seen := make([][]bool, rows)
		for r := range seen {
			seen[r] = make([]bool, cols)
		}
		var stack [][2]int
		for _, s := range starts {
			if !seen[s[0]][s[1]] {
				seen[s[0]][s[1]] = true
				stack = append(stack, s)
			}
		}
		for len(stack) > 0 {
			cell := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			r, c := cell[0], cell[1]
			for _, d := range dirs {
				nr, nc := r+d[0], c+d[1]
				if nr >= 0 && nr < rows && nc >= 0 && nc < cols && !seen[nr][nc] &&
					heights[nr][nc] >= heights[r][c] {
					seen[nr][nc] = true
					stack = append(stack, [2]int{nr, nc})
				}
			}
		}
		return seen
	}

	var pacificStarts, atlanticStarts [][2]int
	for c := 0; c < cols; c++ {
		pacificStarts = append(pacificStarts, [2]int{0, c})
		atlanticStarts = append(atlanticStarts, [2]int{rows - 1, c})
	}
	for r := 0; r < rows; r++ {
		pacificStarts = append(pacificStarts, [2]int{r, 0})
		atlanticStarts = append(atlanticStarts, [2]int{r, cols - 1})
	}
	pacific := reachable(pacificStarts)
	atlantic := reachable(atlanticStarts)
	var result [][]int
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if pacific[r][c] && atlantic[r][c] {
				result = append(result, []int{r, c})
			}
		}
	}
	return result
}
