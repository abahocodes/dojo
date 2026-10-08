package main

func maximalSquare(matrix [][]string) int {
	cols := len(matrix[0])
	side := make([]int, cols+1)
	best := 0
	for _, row := range matrix {
		prevDiag := 0
		for c := 1; c <= cols; c++ {
			above := side[c]
			if row[c-1] == "1" {
				side[c] = 1 + min(above, side[c-1], prevDiag)
				best = max(best, side[c])
			} else {
				side[c] = 0
			}
			prevDiag = above
		}
	}
	return best * best
}
