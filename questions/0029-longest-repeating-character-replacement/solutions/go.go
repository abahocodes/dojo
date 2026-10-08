package main

func characterReplacement(s string, k int) int {
	var counts [26]int
	maxFreq := 0
	left := 0
	for right := 0; right < len(s); right++ {
		idx := s[right] - 'A'
		counts[idx]++
		if counts[idx] > maxFreq {
			maxFreq = counts[idx]
		}
		if right-left+1-maxFreq > k {
			counts[s[left]-'A']--
			left++
		}
	}
	return len(s) - left
}
