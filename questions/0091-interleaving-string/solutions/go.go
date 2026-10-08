package main

func isInterleave(s1 string, s2 string, s3 string) bool {
	m, n := len(s1), len(s2)
	if m+n != len(s3) {
		return false
	}
	ok := make([]bool, n+1)
	for i := 0; i <= m; i++ {
		for j := 0; j <= n; j++ {
			if i == 0 && j == 0 {
				ok[j] = true
				continue
			}
			c := s3[i+j-1]
			fromS1 := i > 0 && ok[j] && s1[i-1] == c
			fromS2 := j > 0 && ok[j-1] && s2[j-1] == c
			ok[j] = fromS1 || fromS2
		}
	}
	return ok[n]
}
