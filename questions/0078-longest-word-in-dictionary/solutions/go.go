package main

type wordTrie struct {
	children map[byte]*wordTrie
	word     string
	isWord   bool
}

func longestWord(words []string) string {
	trie := &wordTrie{children: map[byte]*wordTrie{}}
	for _, word := range words {
		node := trie
		for i := 0; i < len(word); i++ {
			next, ok := node.children[word[i]]
			if !ok {
				next = &wordTrie{children: map[byte]*wordTrie{}}
				node.children[word[i]] = next
			}
			node = next
		}
		node.word = word
		node.isWord = true
	}

	best := ""
	stack := []*wordTrie{trie}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, child := range node.children {
			if !child.isWord {
				continue // only walk through prefixes that are words themselves
			}
			word := child.word
			if len(word) > len(best) || (len(word) == len(best) && word < best) {
				best = word
			}
			stack = append(stack, child)
		}
	}
	return best
}
