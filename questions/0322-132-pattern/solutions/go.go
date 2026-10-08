package main

import "math"

func find132Pattern(nums []int) bool {
	third := math.MinInt64
	stack := make([]int, 0, len(nums))
	for i := len(nums) - 1; i >= 0; i-- {
		x := nums[i]
		if x < third {
			return true
		}
		for len(stack) > 0 && stack[len(stack)-1] < x {
			third = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, x)
	}
	return false
}
