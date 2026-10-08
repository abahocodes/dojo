package main

func calculate(s string) int {
	result, num, sign := 0, 0, 1
	stack := []int{}
	for i := 0; i < len(s); i++ {
		ch := s[i]
		switch {
		case ch >= '0' && ch <= '9':
			num = num*10 + int(ch-'0')
		case ch == '+' || ch == '-':
			result += sign * num
			num = 0
			if ch == '+' {
				sign = 1
			} else {
				sign = -1
			}
		case ch == '(':
			stack = append(stack, result, sign)
			result, sign = 0, 1
		case ch == ')':
			result += sign * num
			num = 0
			savedSign := stack[len(stack)-1]
			savedResult := stack[len(stack)-2]
			stack = stack[:len(stack)-2]
			result = savedResult + savedSign*result
		}
	}
	return result + sign*num
}
