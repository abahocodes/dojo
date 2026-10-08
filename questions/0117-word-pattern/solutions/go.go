package main

import "strings"

func wordPattern(pattern string, s string) bool {
	words := strings.Split(s, " ")
	if len(words) != len(pattern) {
		return false
	}
	letterToWord := map[byte]string{}
	wordToLetter := map[string]byte{}
	for i, w := range words {
		c := pattern[i]
		if prev, ok := letterToWord[c]; ok {
			if prev != w {
				return false
			}
			continue
		}
		if _, taken := wordToLetter[w]; taken {
			return false
		}
		letterToWord[c] = w
		wordToLetter[w] = c
	}
	return true
}
