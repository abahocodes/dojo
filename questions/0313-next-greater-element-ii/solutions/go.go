package main

func nextGreaterCircular(nums []int) []int {
	n := len(nums)
	result := make([]int, n)
	for i := range result {
		result[i] = -1
	}
	stack := make([]int, 0, n)
	for j := 0; j < 2*n; j++ {
		x := nums[j%n]
		for len(stack) > 0 && nums[stack[len(stack)-1]] < x {
			result[stack[len(stack)-1]] = x
			stack = stack[:len(stack)-1]
		}
		if j < n {
			stack = append(stack, j)
		}
	}
	return result
}
