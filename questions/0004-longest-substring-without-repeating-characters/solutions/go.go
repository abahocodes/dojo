package main

func lengthOfLongestSubstring(s string) int {
	last := make(map[rune]int)
	left, best := 0, 0
	for i, ch := range []rune(s) {
		if prev, ok := last[ch]; ok && prev >= left {
			left = prev + 1
		}
		last[ch] = i
		best = max(best, i-left+1)
	}
	return best
}
