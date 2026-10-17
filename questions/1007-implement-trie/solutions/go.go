package main

type trieNode struct {
	next  [26]*trieNode
	isEnd bool
}

type Trie struct {
	root *trieNode
}

func Constructor() Trie {
	return Trie{root: &trieNode{}}
}

func (t *Trie) Insert(word string) {
	node := t.root
	for i := 0; i < len(word); i++ {
		c := word[i] - 'a'
		if node.next[c] == nil {
			node.next[c] = &trieNode{}
		}
		node = node.next[c]
	}
	node.isEnd = true
}

// walk returns the node spelled by s, or nil if the path leaves the trie.
func (t *Trie) walk(s string) *trieNode {
	node := t.root
	for i := 0; i < len(s) && node != nil; i++ {
		node = node.next[s[i]-'a']
	}
	return node
}

func (t *Trie) Search(word string) bool {
	node := t.walk(word)
	return node != nil && node.isEnd
}

func (t *Trie) StartsWith(prefix string) bool {
	return t.walk(prefix) != nil
}

// dojo calls this to run the tests; you don't need to change it.
func runTrie(ops []string, words []string) []bool {
	trie := Constructor()
	out := []bool{}
	for i, op := range ops {
		switch op {
		case "insert":
			trie.Insert(words[i])
		case "search":
			out = append(out, trie.Search(words[i]))
		default:
			out = append(out, trie.StartsWith(words[i]))
		}
	}
	return out
}
