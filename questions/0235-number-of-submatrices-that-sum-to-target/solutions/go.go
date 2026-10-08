package main

func numSubmatrixSumTarget(matrix [][]int, target int) int {
	rows, cols := len(matrix), len(matrix[0])
	count := 0
	for top := 0; top < rows; top++ {
		col := make([]int, cols)
		for bottom := top; bottom < rows; bottom++ {
			for c := 0; c < cols; c++ {
				col[c] += matrix[bottom][c]
			}
			seen := map[int]int{0: 1}
			s := 0
			for _, v := range col {
				s += v
				count += seen[s-target]
				seen[s]++
			}
		}
	}
	return count
}
