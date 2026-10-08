package main

import "strings"

func maxScoreSplit(s string) int {
	score := strings.Count(s, "1")
	best := 0
	for i := 0; i < len(s)-1; i++ {
		if s[i] == '0' {
			score++
		} else {
			score--
		}
		if score > best {
			best = score
		}
	}
	return best
}
