package main

func maxCount(counts []int) int {
	best := 0
	for _, x := range counts {
		if x > best {
			best = x
		}
	}
	return best
}

func reorganizeString(s string) string {
	counts := make([]int, 26)
	for i := 0; i < len(s); i++ {
		counts[s[i]-'a']++
	}
	n := len(s)
	if maxCount(counts) > (n+1)/2 {
		return ""
	}

	result := make([]byte, 0, n)
	prev := -1
	for pos := 0; pos < n; pos++ {
		rest := n - pos - 1 // letters left after placing this one
		for c := 0; c < 26; c++ {
			if counts[c] == 0 || c == prev {
				continue
			}
			counts[c]--
			// The rest can follow c iff no letter needs more than half the
			// remaining slots, and c itself cannot take the very next slot.
			if counts[c] <= rest/2 && maxCount(counts) <= (rest+1)/2 {
				result = append(result, byte('a'+c))
				prev = c
				break
			}
			counts[c]++
		}
	}
	return string(result)
}
