package main

func streamChecker(words []string, stream string) []bool {
	// Trie of reversed words: child[node][c], 0 means "no child".
	child := [][26]int{{}}
	isEnd := []bool{false}
	longest := 0
	for _, word := range words {
		if len(word) > longest {
			longest = len(word)
		}
		node := 0
		for i := len(word) - 1; i >= 0; i-- {
			c := word[i] - 'a'
			if child[node][c] == 0 {
				child[node][c] = len(child)
				child = append(child, [26]int{})
				isEnd = append(isEnd, false)
			}
			node = child[node][c]
		}
		isEnd[node] = true
	}

	result := make([]bool, len(stream))
	for i := 0; i < len(stream); i++ {
		node := 0
		for j := i; j >= 0 && j > i-longest; j-- {
			node = child[node][stream[j]-'a']
			if node == 0 {
				break
			}
			if isEnd[node] {
				result[i] = true
				break
			}
		}
	}
	return result
}
