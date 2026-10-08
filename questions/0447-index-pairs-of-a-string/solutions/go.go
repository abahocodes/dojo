package main

func indexPairs(text string, words []string) [][]int {
	// Trie with 26-way child arrays; end[node] marks the end of a word.
	next := [][26]int{{}}
	end := []bool{false}
	for _, word := range words {
		node := 0
		for k := 0; k < len(word); k++ {
			c := word[k] - 'a'
			if next[node][c] == 0 {
				next[node][c] = len(next)
				next = append(next, [26]int{})
				end = append(end, false)
			}
			node = next[node][c]
		}
		end[node] = true
	}

	result := [][]int{}
	for i := 0; i < len(text); i++ {
		node := 0
		for j := i; j < len(text); j++ {
			node = next[node][text[j]-'a']
			if node == 0 {
				break
			}
			if end[node] {
				result = append(result, []int{i, j})
			}
		}
	}
	return result
}
