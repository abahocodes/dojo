package main

import "sort"

func findAllConcatenatedWords(words []string) []string {
	order := make([]int, len(words))
	for i := range order {
		order[i] = i
	}
	sort.SliceStable(order, func(a, b int) bool { return len(words[order[a]]) < len(words[order[b]]) })
	known := make(map[string]bool)
	found := make([]bool, len(words))
	for _, i := range order {
		w := words[i]
		if len(known) > 0 {
			n := len(w)
			can := make([]bool, n+1)
			can[0] = true
			for end := 1; end <= n; end++ {
				for start := 0; start < end; start++ {
					if can[start] && known[w[start:end]] {
						can[end] = true
						break
					}
				}
			}
			found[i] = can[n]
		}
		known[w] = true
	}
	result := []string{}
	for i, w := range words {
		if found[i] {
			result = append(result, w)
		}
	}
	return result
}
