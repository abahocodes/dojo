package main

import "sort"

func topKFrequentWords(words []string, k int) []string {
	counts := make(map[string]int)
	for _, w := range words {
		counts[w]++
	}
	ranked := make([]string, 0, len(counts))
	for w := range counts {
		ranked = append(ranked, w)
	}
	sort.Slice(ranked, func(a, b int) bool {
		ca, cb := counts[ranked[a]], counts[ranked[b]]
		if ca != cb {
			return ca > cb
		}
		return ranked[a] < ranked[b]
	})
	return ranked[:k]
}
