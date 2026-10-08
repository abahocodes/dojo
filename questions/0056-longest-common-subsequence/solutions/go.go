package main

func longestCommonSubsequence(a string, b string) int {
	m := len(b)
	prev := make([]int, m+1)
	for i := 0; i < len(a); i++ {
		curr := make([]int, m+1)
		for j := 1; j <= m; j++ {
			if a[i] == b[j-1] {
				curr[j] = prev[j-1] + 1
			} else {
				curr[j] = max(prev[j], curr[j-1])
			}
		}
		prev = curr
	}
	return prev[m]
}
