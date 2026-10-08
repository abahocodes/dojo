package main

func wordSquares(words []string) [][]string {
	size := len(words[0])
	// Every prefix (including the empty one) -> the words starting with it.
	byPrefix := map[string][]string{}
	for _, word := range words {
		for i := 0; i <= size; i++ {
			byPrefix[word[:i]] = append(byPrefix[word[:i]], word)
		}
	}

	result := [][]string{}
	square := []string{}
	var backtrack func()
	backtrack = func() {
		k := len(square)
		if k == size {
			result = append(result, append([]string{}, square...))
			return
		}
		// Row k must start with column k of the rows placed so far.
		prefix := make([]byte, 0, k)
		for _, row := range square {
			prefix = append(prefix, row[k])
		}
		for _, word := range byPrefix[string(prefix)] {
			square = append(square, word)
			backtrack()
			square = square[:len(square)-1]
		}
	}

	backtrack()
	return result
}
