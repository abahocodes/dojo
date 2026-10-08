package main

func sumSubarrayMins(arr []int) int {
	const mod = 1_000_000_007
	n := len(arr)
	stack := make([]int, 0, n+1)
	total := 0
	for j := 0; j <= n; j++ {
		cur := 0
		if j < n {
			cur = arr[j]
		}
		for len(stack) > 0 && arr[stack[len(stack)-1]] >= cur {
			i := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}
			total = (total + arr[i]*(i-left)%mod*(j-i)) % mod
		}
		stack = append(stack, j)
	}
	return total
}
