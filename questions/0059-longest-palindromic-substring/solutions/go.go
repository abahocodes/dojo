package main

func longestPalindrome(s string) string {
	n := len(s)
	bestLo, bestLen := 0, 1
	for center := 0; center < n; center++ {
		for _, start := range [2][2]int{{center, center}, {center, center + 1}} {
			lo, hi := start[0], start[1]
			for lo >= 0 && hi < n && s[lo] == s[hi] {
				lo--
				hi++
			}
			if length := hi - lo - 1; length > bestLen {
				bestLo, bestLen = lo+1, length
			}
		}
	}
	return s[bestLo : bestLo+bestLen]
}
