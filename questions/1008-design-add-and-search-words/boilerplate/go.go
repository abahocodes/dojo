// 1008. Design Add and Search Words Data Structure (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type WordDictionary struct {
}

func Constructor() WordDictionary {
	return WordDictionary{}
}

func (d *WordDictionary) AddWord(word string) {

}

func (d *WordDictionary) Search(word string) bool {
	return false
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
