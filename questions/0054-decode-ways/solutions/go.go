package main

func numDecodings(s string) int {
	prev, curr := 1, 0
	if s[0] != '0' {
		curr = 1
	}
	for i := 2; i <= len(s); i++ {
		next := 0
		if s[i-1] != '0' {
			next += curr
		}
		pair := int(s[i-2]-'0')*10 + int(s[i-1]-'0')
		if pair >= 10 && pair <= 26 {
			next += prev
		}
		prev, curr = curr, next
	}
	return curr
}
