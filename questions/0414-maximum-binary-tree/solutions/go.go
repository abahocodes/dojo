package main

func constructMaximumBinaryTree(nums []int) *TreeNode {
	stack := []*TreeNode{}
	for _, x := range nums {
		node := &TreeNode{Val: x}
		var last *TreeNode
		for len(stack) > 0 && stack[len(stack)-1].Val < x {
			last = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		node.Left = last
		if len(stack) > 0 {
			stack[len(stack)-1].Right = node
		}
		stack = append(stack, node)
	}
	return stack[0]
}
