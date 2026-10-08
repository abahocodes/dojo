package main

import "math"

func verifyPreorder(preorder []int) bool {
	low := math.MinInt
	var stack []int
	for _, x := range preorder {
		if x < low {
			return false
		}
		for len(stack) > 0 && stack[len(stack)-1] < x {
			low = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, x)
	}
	return true
}
