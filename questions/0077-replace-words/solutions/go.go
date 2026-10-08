package main

import "strings"

type rootTrie struct {
	children [26]*rootTrie
	end      bool
}

func replaceWords(roots []string, sentence string) string {
	trie := &rootTrie{}
	for _, root := range roots {
		node := trie
		for i := 0; i < len(root); i++ {
			k := root[i] - 'a'
			if node.children[k] == nil {
				node.children[k] = &rootTrie{}
			}
			node = node.children[k]
		}
		node.end = true
	}

	shortestRoot := func(word string) string {
		node := trie
		for i := 0; i < len(word); i++ {
			node = node.children[word[i]-'a']
			if node == nil {
				return word
			}
			if node.end {
				return word[:i+1]
			}
		}
		return word
	}

	words := strings.Split(sentence, " ")
	for i, word := range words {
		words[i] = shortestRoot(word)
	}
	return strings.Join(words, " ")
}
