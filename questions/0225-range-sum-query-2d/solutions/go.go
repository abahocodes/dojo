package main

func regionSums(matrix [][]int, queries [][]int) []int {
	m, n := len(matrix), len(matrix[0])
	// pre[i][j] = sum of matrix[0..i-1][0..j-1]
	pre := make([][]int, m+1)
	for i := range pre {
		pre[i] = make([]int, n+1)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			pre[i+1][j+1] = matrix[i][j] + pre[i][j+1] + pre[i+1][j] - pre[i][j]
		}
	}
	result := make([]int, len(queries))
	for q, qu := range queries {
		r1, c1, r2, c2 := qu[0], qu[1], qu[2], qu[3]
		result[q] = pre[r2+1][c2+1] - pre[r1][c2+1] - pre[r2+1][c1] + pre[r1][c1]
	}
	return result
}
