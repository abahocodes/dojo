package main

import "strings"

func removeKDuplicates(s string, k int) string {
	letters := make([]byte, 0, len(s))
	counts := make([]int, 0, len(s))
	for i := 0; i < len(s); i++ {
		ch := s[i]
		top := len(letters) - 1
		if top >= 0 && letters[top] == ch {
			counts[top]++
			if counts[top] == k {
				letters = letters[:top]
				counts = counts[:top]
			}
		} else {
			letters = append(letters, ch)
			counts = append(counts, 1)
		}
	}
	var sb strings.Builder
	for i, ch := range letters {
		for c := 0; c < counts[i]; c++ {
			sb.WriteByte(ch)
		}
	}
	return sb.String()
}
