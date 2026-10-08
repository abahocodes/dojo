package main

func matrixBlockSum(mat [][]int, k int) [][]int {
	m, n := len(mat), len(mat[0])
	pre := make([][]int, m+1)
	for i := range pre {
		pre[i] = make([]int, n+1)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			pre[i+1][j+1] = mat[i][j] + pre[i][j+1] + pre[i+1][j] - pre[i][j]
		}
	}
	result := make([][]int, m)
	for i := 0; i < m; i++ {
		r1, r2 := max(0, i-k), min(m, i+k+1)
		result[i] = make([]int, n)
		for j := 0; j < n; j++ {
			c1, c2 := max(0, j-k), min(n, j+k+1)
			result[i][j] = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1]
		}
	}
	return result
}
