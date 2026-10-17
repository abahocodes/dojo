package main

func checkValidString(s string) bool {
	// lo..hi is the range of possible counts of unmatched '(' so far.
	lo, hi := 0, 0
	for _, ch := range s {
		switch ch {
		case '(':
			lo++
			hi++
		case ')':
			lo--
			hi--
		default: // '*' can be ')', nothing, or '('
			lo--
			hi++
		}
		if hi < 0 {
			return false
		}
		if lo < 0 {
			lo = 0
		}
	}
	return lo == 0
}
