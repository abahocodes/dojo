package main

func findMode(root *TreeNode) []int {
	var modes []int
	best, count, prev := 0, 0, 0
	hasPrev := false
	var stack []*TreeNode
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if hasPrev && node.Val == prev {
			count++
		} else {
			count = 1
		}
		prev, hasPrev = node.Val, true
		if count > best {
			best = count
			modes = []int{node.Val}
		} else if count == best {
			modes = append(modes, node.Val)
		}
		node = node.Right
	}
	return modes
}
