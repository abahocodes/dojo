package main

func searchMatrix(matrix [][]int, target int) bool {
	cols := len(matrix[0])
	lo, hi := 0, len(matrix)*cols-1
	for lo <= hi {
		mid := (lo + hi) / 2
		value := matrix[mid/cols][mid%cols]
		if value == target {
			return true
		}
		if value < target {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}
