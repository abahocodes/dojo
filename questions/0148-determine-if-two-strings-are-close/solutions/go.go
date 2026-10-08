package main

import "sort"

func closeStrings(word1 string, word2 string) bool {
	if len(word1) != len(word2) {
		return false
	}
	a := make([]int, 26)
	b := make([]int, 26)
	for i := 0; i < len(word1); i++ {
		a[word1[i]-'a']++
	}
	for i := 0; i < len(word2); i++ {
		b[word2[i]-'a']++
	}
	for i := 0; i < 26; i++ {
		if (a[i] == 0) != (b[i] == 0) {
			return false
		}
	}
	sort.Ints(a)
	sort.Ints(b)
	for i := 0; i < 26; i++ {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
