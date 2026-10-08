package main

func removeDuplicates(s string) string {
	stack := make([]byte, 0, len(s))
	for i := 0; i < len(s); i++ {
		if n := len(stack); n > 0 && stack[n-1] == s[i] {
			stack = stack[:n-1]
		} else {
			stack = append(stack, s[i])
		}
	}
	return string(stack)
}
