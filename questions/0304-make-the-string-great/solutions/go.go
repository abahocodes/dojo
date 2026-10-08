package main

func makeGood(s string) string {
	stack := make([]byte, 0, len(s))
	for i := 0; i < len(s); i++ {
		ch := s[i]
		// 'a' and 'A' differ by exactly 32 in ASCII.
		if n := len(stack); n > 0 && (stack[n-1]^ch) == 32 {
			stack = stack[:n-1]
		} else {
			stack = append(stack, ch)
		}
	}
	return string(stack)
}
