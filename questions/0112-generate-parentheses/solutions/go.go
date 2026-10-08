package main

func generateParenthesis(n int) []string {
	var result []string
	current := make([]byte, 0, 2*n)

	var backtrack func(open, close int)
	backtrack = func(open, close int) {
		if len(current) == 2*n {
			result = append(result, string(current))
			return
		}
		if open < n {
			current = append(current, '(')
			backtrack(open+1, close)
			current = current[:len(current)-1]
		}
		if close < open {
			current = append(current, ')')
			backtrack(open, close+1)
			current = current[:len(current)-1]
		}
	}

	backtrack(0, 0)
	return result
}
