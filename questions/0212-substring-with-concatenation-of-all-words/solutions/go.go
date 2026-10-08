package main

import "sort"

func findSubstring(s string, words []string) []int {
	L, m := len(words[0]), len(words)
	result := []int{}
	if m*L > len(s) {
		return result
	}
	need := map[string]int{}
	for _, w := range words {
		need[w]++
	}
	for r := 0; r < L; r++ {
		have := map[string]int{}
		left, count := r, 0
		for right := r; right+L <= len(s); right += L {
			w := s[right : right+L]
			if _, ok := need[w]; !ok {
				have = map[string]int{}
				count = 0
				left = right + L
				continue
			}
			have[w]++
			count++
			for have[w] > need[w] {
				have[s[left:left+L]]--
				count--
				left += L
			}
			if count == m {
				result = append(result, left)
				have[s[left:left+L]]--
				count--
				left += L
			}
		}
	}
	sort.Ints(result)
	return result
}
