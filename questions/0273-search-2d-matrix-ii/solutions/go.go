package main

func searchMatrixSorted(matrix [][]int, target int) bool {
	rows := len(matrix)
	r, c := 0, len(matrix[0])-1
	for r < rows && c >= 0 {
		value := matrix[r][c]
		if value == target {
			return true
		}
		if value > target {
			c--
		} else {
			r++
		}
	}
	return false
}
