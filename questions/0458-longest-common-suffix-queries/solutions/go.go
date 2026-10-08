package main

func stringIndices(wordsContainer []string, wordsQuery []string) []int {
	total := 0
	for _, w := range wordsContainer {
		total += len(w)
	}
	children := make([]int32, (total+1)*26)
	best := make([]int, total+1)
	nodes := int32(1)
	for i, w := range wordsContainer {
		if len(w) < len(wordsContainer[best[0]]) {
			best[0] = i
		}
		node := int32(0)
		for k := len(w) - 1; k >= 0; k-- {
			slot := int(node)*26 + int(w[k]-'a')
			if children[slot] == 0 {
				children[slot] = nodes
				best[nodes] = i
				nodes++
			} else if len(w) < len(wordsContainer[best[children[slot]]]) {
				best[children[slot]] = i
			}
			node = children[slot]
		}
	}
	result := make([]int, len(wordsQuery))
	for j, q := range wordsQuery {
		node := int32(0)
		for k := len(q) - 1; k >= 0; k-- {
			next := children[int(node)*26+int(q[k]-'a')]
			if next == 0 {
				break
			}
			node = next
		}
		result[j] = best[node]
	}
	return result
}
