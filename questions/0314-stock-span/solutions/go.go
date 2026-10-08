package main

func stockSpan(prices []int) []int {
	n := len(prices)
	result := make([]int, n)
	stack := make([]int, 0, n)
	for i := 0; i < n; i++ {
		for len(stack) > 0 && prices[stack[len(stack)-1]] <= prices[i] {
			stack = stack[:len(stack)-1]
		}
		if len(stack) > 0 {
			result[i] = i - stack[len(stack)-1]
		} else {
			result[i] = i + 1
		}
		stack = append(stack, i)
	}
	return result
}
