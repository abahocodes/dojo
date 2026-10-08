package main

func lengthOfLongestSubstringKDistinct(s string, k int) int {
	var count [128]int
	distinct, left, best := 0, 0, 0
	for right := 0; right < len(s); right++ {
		if count[s[right]] == 0 {
			distinct++
		}
		count[s[right]]++
		for distinct > k {
			count[s[left]]--
			if count[s[left]] == 0 {
				distinct--
			}
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}
