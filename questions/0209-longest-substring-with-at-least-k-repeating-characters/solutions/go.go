package main

func longestSubstringKRepeating(s string, k int) int {
	best := 0
	for limit := 1; limit <= 26; limit++ {
		var count [26]int
		left, unique, atLeast := 0, 0, 0
		for right := 0; right < len(s); right++ {
			c := int(s[right] - 'a')
			if count[c] == 0 {
				unique++
			}
			count[c]++
			if count[c] == k {
				atLeast++
			}
			for unique > limit {
				d := int(s[left] - 'a')
				if count[d] == k {
					atLeast--
				}
				count[d]--
				if count[d] == 0 {
					unique--
				}
				left++
			}
			if unique == atLeast && right-left+1 > best {
				best = right - left + 1
			}
		}
	}
	return best
}
