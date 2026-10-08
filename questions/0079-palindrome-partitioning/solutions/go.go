package main

func partition(s string) [][]string {
	n := len(s)
	// pal[i][j] is true when s[i..j] is a palindrome
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			if s[i] == s[j] && (j-i < 2 || pal[i+1][j-1]) {
				pal[i][j] = true
			}
		}
	}

	result := [][]string{}
	current := []string{}

	var backtrack func(start int)
	backtrack = func(start int) {
		if start == n {
			result = append(result, append([]string(nil), current...))
			return
		}
		for end := start; end < n; end++ {
			if pal[start][end] {
				current = append(current, s[start:end+1])
				backtrack(end + 1)
				current = current[:len(current)-1]
			}
		}
	}

	backtrack(0)
	return result
}
