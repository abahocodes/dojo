package main

func removeKdigits(num string, k int) string {
	stack := make([]byte, 0, len(num))
	for i := 0; i < len(num); i++ {
		d := num[i]
		for k > 0 && len(stack) > 0 && stack[len(stack)-1] > d {
			stack = stack[:len(stack)-1]
			k--
		}
		stack = append(stack, d)
	}
	stack = stack[:len(stack)-k]
	start := 0
	for start < len(stack) && stack[start] == '0' {
		start++
	}
	if start == len(stack) {
		return "0"
	}
	return string(stack[start:])
}
