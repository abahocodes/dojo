package main

func convertBst(root *TreeNode) *TreeNode {
	running := 0
	var stack []*TreeNode
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Right
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		running += node.Val
		node.Val = running
		node = node.Left
	}
	return root
}
