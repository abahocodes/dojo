package main

func minAddToMakeValid(s string) int {
	open, added := 0, 0
	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			open++
		} else if open > 0 {
			open--
		} else {
			added++
		}
	}
	return added + open
}
