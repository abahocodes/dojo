package main

func minimumLengthEncoding(words []string) int {
	keep := make(map[string]bool)
	for _, w := range words {
		keep[w] = true
	}
	for _, w := range words {
		for k := 1; k < len(w); k++ {
			delete(keep, w[k:])
		}
	}
	total := 0
	for w := range keep {
		total += len(w) + 1
	}
	return total
}
