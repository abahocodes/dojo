package main

func minRemoveToMakeValid(s string) string {
	n := len(s)
	removed := make([]bool, n)
	stack := make([]int, 0)
	for i := 0; i < n; i++ {
		if s[i] == '(' {
			stack = append(stack, i)
		} else if s[i] == ')' {
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			} else {
				removed[i] = true
			}
		}
	}
	for _, i := range stack {
		removed[i] = true
	}
	out := make([]byte, 0, n)
	for i := 0; i < n; i++ {
		if !removed[i] {
			out = append(out, s[i])
		}
	}
	return string(out)
}
