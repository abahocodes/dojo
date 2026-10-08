package main

import "strings"

func customSortString(order string, s string) string {
	var counts [26]int
	for i := 0; i < len(s); i++ {
		counts[s[i]-'a']++
	}
	var ranked [26]bool
	var sb strings.Builder
	for i := 0; i < len(order); i++ {
		c := order[i]
		ranked[c-'a'] = true
		for k := 0; k < counts[c-'a']; k++ {
			sb.WriteByte(c)
		}
	}
	for i := 0; i < len(s); i++ {
		if !ranked[s[i]-'a'] {
			sb.WriteByte(s[i])
		}
	}
	return sb.String()
}
