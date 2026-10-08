package main

import "math"

func getMinimumDifference(root *TreeNode) int {
	best := math.MaxInt
	hasPrev, prev := false, 0
	var stack []*TreeNode
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if hasPrev && node.Val-prev < best {
			best = node.Val - prev
		}
		prev, hasPrev = node.Val, true
		node = node.Right
	}
	return best
}
