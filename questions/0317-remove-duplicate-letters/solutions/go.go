package main

func removeDuplicateLetters(s string) string {
	var last [26]int
	for i := 0; i < len(s); i++ {
		last[s[i]-'a'] = i
	}
	var used [26]bool
	stack := make([]byte, 0, 26)
	for i := 0; i < len(s); i++ {
		c := s[i]
		if used[c-'a'] {
			continue
		}
		for len(stack) > 0 && stack[len(stack)-1] > c && last[stack[len(stack)-1]-'a'] > i {
			used[stack[len(stack)-1]-'a'] = false
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, c)
		used[c-'a'] = true
	}
	return string(stack)
}
