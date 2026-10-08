package main

func reverseParentheses(s string) string {
	n := len(s)
	partner := make([]int, n)
	opens := []int{}
	for i := 0; i < n; i++ {
		if s[i] == '(' {
			opens = append(opens, i)
		} else if s[i] == ')' {
			j := opens[len(opens)-1]
			opens = opens[:len(opens)-1]
			partner[i] = j
			partner[j] = i
		}
	}
	out := make([]byte, 0, n)
	step := 1
	for i := 0; i < n; i += step {
		if s[i] == '(' || s[i] == ')' {
			i = partner[i]
			step = -step
		} else {
			out = append(out, s[i])
		}
	}
	return string(out)
}
