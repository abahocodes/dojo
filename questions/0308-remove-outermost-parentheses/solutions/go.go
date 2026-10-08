package main

import "strings"

func removeOuterParentheses(s string) string {
	var out strings.Builder
	depth := 0
	for i := 0; i < len(s); i++ {
		ch := s[i]
		if ch == '(' {
			if depth > 0 {
				out.WriteByte(ch)
			}
			depth++
		} else {
			depth--
			if depth > 0 {
				out.WriteByte(ch)
			}
		}
	}
	return out.String()
}
