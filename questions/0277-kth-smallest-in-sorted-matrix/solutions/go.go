package main

func kthSmallestMatrix(matrix [][]int, k int) int {
	n := len(matrix)
	// Staircase walk from the bottom-left corner.
	countAtMost := func(v int) int {
		count, row, col := 0, n-1, 0
		for row >= 0 && col < n {
			if matrix[row][col] <= v {
				count += row + 1
				col++
			} else {
				row--
			}
		}
		return count
	}
	lo, hi := matrix[0][0], matrix[n-1][n-1]
	for lo < hi {
		mid := lo + (hi-lo)/2
		if countAtMost(mid) >= k {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
