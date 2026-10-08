package main

import "strings"

func mergeAlternately(word1 string, word2 string) string {
	var out strings.Builder
	out.Grow(len(word1) + len(word2))
	for i := 0; i < len(word1) || i < len(word2); i++ {
		if i < len(word1) {
			out.WriteByte(word1[i])
		}
		if i < len(word2) {
			out.WriteByte(word2[i])
		}
	}
	return out.String()
}
