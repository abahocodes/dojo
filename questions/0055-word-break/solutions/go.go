package main

import "sort"

func wordBreak(s string, words []string) bool {
	vocab := make(map[string]bool, len(words))
	seenLength := make(map[int]bool)
	var lengths []int
	for _, w := range words {
		vocab[w] = true
		if !seenLength[len(w)] {
			seenLength[len(w)] = true
			lengths = append(lengths, len(w))
		}
	}
	sort.Ints(lengths)
	ok := make([]bool, len(s)+1)
	ok[0] = true
	for i := 1; i <= len(s); i++ {
		for _, length := range lengths {
			if length > i {
				break
			}
			if ok[i-length] && vocab[s[i-length:i]] {
				ok[i] = true
				break
			}
		}
	}
	return ok[len(s)]
}
