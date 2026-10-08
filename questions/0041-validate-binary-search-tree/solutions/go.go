package main

func isValidBst(root *TreeNode) bool {
	stack := []*TreeNode{}
	var prev *int
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if prev != nil && node.Val <= *prev {
			return false
		}
		val := node.Val
		prev = &val
		node = node.Right
	}
	return true
}
