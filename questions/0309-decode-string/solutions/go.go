package main

import "strings"

func decodeString(s string) string {
	type frame struct {
		prefix string
		times  int
	}
	var stack []frame
	buf := ""
	k := 0
	for i := 0; i < len(s); i++ {
		ch := s[i]
		switch {
		case ch >= '0' && ch <= '9':
			k = k*10 + int(ch-'0')
		case ch == '[':
			stack = append(stack, frame{buf, k})
			buf, k = "", 0
		case ch == ']':
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			buf = top.prefix + strings.Repeat(buf, top.times)
		default:
			buf += string(ch)
		}
	}
	return buf
}
