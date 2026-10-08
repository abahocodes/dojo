package main

func rangeSumBst(root *TreeNode, low int, high int) int {
	total := 0
	var stack []*TreeNode
	if root != nil {
		stack = append(stack, root)
	}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		switch {
		case node.Val < low:
			if node.Right != nil {
				stack = append(stack, node.Right)
			}
		case node.Val > high:
			if node.Left != nil {
				stack = append(stack, node.Left)
			}
		default:
			total += node.Val
			if node.Left != nil {
				stack = append(stack, node.Left)
			}
			if node.Right != nil {
				stack = append(stack, node.Right)
			}
		}
	}
	return total
}
