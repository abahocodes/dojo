package main

func sumOfLeftLeaves(root *TreeNode) int {
	total := 0
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if left := node.Left; left != nil {
			if left.Left == nil && left.Right == nil {
				total += left.Val
			} else {
				stack = append(stack, left)
			}
		}
		if node.Right != nil {
			stack = append(stack, node.Right)
		}
	}
	return total
}
