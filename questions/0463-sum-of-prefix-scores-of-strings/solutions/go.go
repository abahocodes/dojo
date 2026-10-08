package main

func sumPrefixScores(words []string) []int {
	// Trie where every node counts how many words pass through it.
	child := [][26]int32{{}}
	count := []int{0}
	for _, word := range words {
		node := int32(0)
		for i := 0; i < len(word); i++ {
			c := word[i] - 'a'
			if child[node][c] == 0 {
				child[node][c] = int32(len(child))
				child = append(child, [26]int32{})
				count = append(count, 0)
			}
			node = child[node][c]
			count[node]++
		}
	}

	result := make([]int, len(words))
	for w, word := range words {
		node, total := int32(0), 0
		for i := 0; i < len(word); i++ {
			node = child[node][word[i]-'a']
			total += count[node]
		}
		result[w] = total
	}
	return result
}
