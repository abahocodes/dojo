package main

import "strings"

func uncommonFromSentences(s1 string, s2 string) []string {
	words := strings.Fields(s1 + " " + s2)
	counts := map[string]int{}
	for _, w := range words {
		counts[w]++
	}
	// Walk the words (not the map) so the output order is deterministic.
	result := []string{}
	for _, w := range words {
		if counts[w] == 1 {
			result = append(result, w)
		}
	}
	return result
}
