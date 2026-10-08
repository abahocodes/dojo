package main

import "strings"

func reverseVowels(s string) string {
	isVowel := func(c byte) bool { return strings.IndexByte("aeiouAEIOU", c) >= 0 }
	chars := []byte(s)
	lo, hi := 0, len(chars)-1
	for lo < hi {
		if !isVowel(chars[lo]) {
			lo++
		} else if !isVowel(chars[hi]) {
			hi--
		} else {
			chars[lo], chars[hi] = chars[hi], chars[lo]
			lo++
			hi--
		}
	}
	return string(chars)
}
