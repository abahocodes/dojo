package main

func findSecondMinimumValue(root *TreeNode) int {
	smallest := root.Val
	best := -1
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if node.Val > smallest {
			if best == -1 || node.Val < best {
				best = node.Val
			}
			continue
		}
		if node.Left != nil {
			stack = append(stack, node.Left, node.Right)
		}
	}
	return best
}
