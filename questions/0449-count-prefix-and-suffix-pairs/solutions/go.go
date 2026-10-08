package main

import "strings"

func countPrefixSuffixPairs(words []string) int {
	count := 0
	for j := range words {
		for i := 0; i < j; i++ {
			if strings.HasPrefix(words[j], words[i]) && strings.HasSuffix(words[j], words[i]) {
				count++
			}
		}
	}
	return count
}
