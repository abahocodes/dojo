package main

func finalPrices(prices []int) []int {
	result := make([]int, len(prices))
	copy(result, prices)
	stack := []int{} // indices awaiting a discount; prices increase
	for j, p := range prices {
		for len(stack) > 0 && prices[stack[len(stack)-1]] >= p {
			result[stack[len(stack)-1]] -= p
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, j)
	}
	return result
}
