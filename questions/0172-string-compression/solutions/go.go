package main

import (
	"strconv"
	"strings"
)

func compress(chars string) string {
	var out strings.Builder
	n := len(chars)
	i := 0
	for i < n {
		j := i
		for j < n && chars[j] == chars[i] {
			j++
		}
		out.WriteByte(chars[i])
		if j-i > 1 {
			out.WriteString(strconv.Itoa(j - i))
		}
		i = j
	}
	return out.String()
}
