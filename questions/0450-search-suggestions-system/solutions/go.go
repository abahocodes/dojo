package main

import (
	"sort"
	"strings"
)

func suggestedProducts(products []string, searchWord string) [][]string {
	ordered := append([]string(nil), products...)
	sort.Strings(ordered)
	result := make([][]string, 0, len(searchWord))
	start := 0
	for k := 1; k <= len(searchWord); k++ {
		prefix := searchWord[:k]
		// First index whose word is >= prefix; longer prefixes never move it back.
		start += sort.SearchStrings(ordered[start:], prefix)
		suggestions := []string{}
		for i := start; i < len(ordered) && i < start+3; i++ {
			if !strings.HasPrefix(ordered[i], prefix) {
				break
			}
			suggestions = append(suggestions, ordered[i])
		}
		result = append(result, suggestions)
	}
	return result
}
