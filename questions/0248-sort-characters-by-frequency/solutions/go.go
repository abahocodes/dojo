package main

import (
	"sort"
	"strings"
)

func frequencySort(s string) string {
	var counts [128]int
	for i := 0; i < len(s); i++ {
		counts[s[i]]++
	}
	chars := []int{}
	for c := 0; c < 128; c++ {
		if counts[c] > 0 {
			chars = append(chars, c)
		}
	}
	sort.Slice(chars, func(i, j int) bool {
		a, b := chars[i], chars[j]
		if counts[a] != counts[b] {
			return counts[a] > counts[b]
		}
		return a < b
	})
	var sb strings.Builder
	sb.Grow(len(s))
	for _, c := range chars {
		sb.WriteString(strings.Repeat(string(rune(c)), counts[c]))
	}
	return sb.String()
}
