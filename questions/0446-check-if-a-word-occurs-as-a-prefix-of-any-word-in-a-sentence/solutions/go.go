package main

import "strings"

func isPrefixOfWord(sentence string, searchWord string) int {
	for i, word := range strings.Split(sentence, " ") {
		if strings.HasPrefix(word, searchWord) {
			return i + 1
		}
	}
	return -1
}
