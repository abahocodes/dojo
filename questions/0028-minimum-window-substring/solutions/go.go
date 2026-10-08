package main

func minWindow(s string, t string) string {
	var need [256]int
	for i := 0; i < len(t); i++ {
		need[t[i]]++
	}
	missing := len(t)
	bestStart, bestLen := 0, len(s)+1
	left := 0
	for right := 0; right < len(s); right++ {
		ch := s[right]
		if need[ch] > 0 {
			missing--
		}
		need[ch]--
		for missing == 0 {
			if right-left+1 < bestLen {
				bestStart, bestLen = left, right-left+1
			}
			out := s[left]
			need[out]++
			if need[out] > 0 {
				missing++
			}
			left++
		}
	}
	if bestLen > len(s) {
		return ""
	}
	return s[bestStart : bestStart+bestLen]
}
