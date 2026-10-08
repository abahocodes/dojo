package main

func maxDepthParens(s string) int {
	depth, best := 0, 0
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '(':
			depth++
			if depth > best {
				best = depth
			}
		case ')':
			depth--
		}
	}
	return best
}
