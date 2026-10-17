package main

type wordNode struct {
	next  [26]*wordNode
	isEnd bool
}

type WordDictionary struct {
	root *wordNode
}

func Constructor() WordDictionary {
	return WordDictionary{root: &wordNode{}}
}

func (d *WordDictionary) AddWord(word string) {
	node := d.root
	for i := 0; i < len(word); i++ {
		c := word[i] - 'a'
		if node.next[c] == nil {
			node.next[c] = &wordNode{}
		}
		node = node.next[c]
	}
	node.isEnd = true
}

func (d *WordDictionary) Search(word string) bool {
	var match func(node *wordNode, i int) bool
	match = func(node *wordNode, i int) bool {
		if i == len(word) {
			return node.isEnd
		}
		if word[i] != '.' {
			child := node.next[word[i]-'a']
			return child != nil && match(child, i+1)
		}
		// A dot can be any letter: try every child.
		for _, child := range node.next {
			if child != nil && match(child, i+1) {
				return true
			}
		}
		return false
	}
	return match(d.root, 0)
}

// dojo calls this to run the tests; you don't need to change it.
func runWordDictionary(ops []string, words []string) []bool {
	wd := Constructor()
	out := []bool{}
	for i, op := range ops {
		if op == "addWord" {
			wd.AddWord(words[i])
		} else {
			out = append(out, wd.Search(words[i]))
		}
	}
	return out
}
