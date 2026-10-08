package main

func shortestUniquePrefixes(words []string) []string {
	children := [][26]int32{{}}
	count := []int{0}
	for _, w := range words {
		node := int32(0)
		for i := 0; i < len(w); i++ {
			c := w[i] - 'a'
			if children[node][c] == 0 {
				children[node][c] = int32(len(children))
				children = append(children, [26]int32{})
				count = append(count, 0)
			}
			node = children[node][c]
			count[node]++
		}
	}
	result := make([]string, len(words))
	for k, w := range words {
		node := int32(0)
		length := len(w)
		for i := 0; i < len(w); i++ {
			node = children[node][w[i]-'a']
			if count[node] == 1 {
				length = i + 1
				break
			}
		}
		result[k] = w[:length]
	}
	return result
}
