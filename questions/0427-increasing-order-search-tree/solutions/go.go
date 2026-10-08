package main

func increasingBst(root *TreeNode) *TreeNode {
	dummy := &TreeNode{}
	tail := dummy
	var stack []*TreeNode
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node.Left = nil
		tail.Right = node
		tail = node
		node = node.Right
	}
	return dummy.Right
}
