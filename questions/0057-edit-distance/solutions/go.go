package main

func editDistance(word1 string, word2 string) int {
	m := len(word2)
	prev := make([]int, m+1)
	for j := range prev {
		prev[j] = j
	}
	for i := 1; i <= len(word1); i++ {
		c1 := word1[i-1]
		curr := make([]int, m+1)
		curr[0] = i
		for j := 1; j <= m; j++ {
			if c1 == word2[j-1] {
				curr[j] = prev[j-1]
			} else {
				curr[j] = 1 + min(prev[j], curr[j-1], prev[j-1])
			}
		}
		prev = curr
	}
	return prev[m]
}
