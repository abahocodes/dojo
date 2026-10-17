// 1007. Implement Trie (Prefix Tree) (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type Trie struct {
}

func Constructor() Trie {
	return Trie{}
}

func (t *Trie) Insert(word string) {

}

func (t *Trie) Search(word string) bool {
	return false
}

func (t *Trie) StartsWith(prefix string) bool {
	return false
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
