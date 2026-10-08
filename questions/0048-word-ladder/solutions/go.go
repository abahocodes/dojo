package main

func ladderLength(beginWord string, endWord string, wordList []string) int {
	words := make(map[string]bool, len(wordList))
	for _, w := range wordList {
		words[w] = true
	}
	if !words[endWord] {
		return 0
	}
	delete(words, beginWord)
	type entry struct {
		word  string
		steps int
	}
	queue := []entry{{beginWord, 1}}
	for head := 0; head < len(queue); head++ {
		cur := queue[head]
		buf := []byte(cur.word)
		for i := range buf {
			original := buf[i]
			for ch := byte('a'); ch <= 'z'; ch++ {
				buf[i] = ch
				candidate := string(buf)
				if words[candidate] {
					if candidate == endWord {
						return cur.steps + 1
					}
					delete(words, candidate) // visited: never enqueue twice
					queue = append(queue, entry{candidate, cur.steps + 1})
				}
			}
			buf[i] = original
		}
	}
	return 0
}
