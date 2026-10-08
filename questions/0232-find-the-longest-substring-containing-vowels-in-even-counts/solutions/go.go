package main

func findTheLongestSubstring(s string) int {
	var first [32]int
	for i := range first {
		first[i] = -2
	}
	first[0] = -1
	mask, best := 0, 0
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case 'a':
			mask ^= 1
		case 'e':
			mask ^= 2
		case 'i':
			mask ^= 4
		case 'o':
			mask ^= 8
		case 'u':
			mask ^= 16
		}
		if first[mask] == -2 {
			first[mask] = i
		} else if i-first[mask] > best {
			best = i - first[mask]
		}
	}
	return best
}
