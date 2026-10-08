package main

func constructFromPrePost(preorder []int, postorder []int) *TreeNode {
	root := &TreeNode{Val: preorder[0]}
	stack := []*TreeNode{root}
	j := 0
	for _, v := range preorder[1:] {
		node := &TreeNode{Val: v}
		// Pop every node whose subtree is already complete.
		for stack[len(stack)-1].Val == postorder[j] {
			stack = stack[:len(stack)-1]
			j++
		}
		parent := stack[len(stack)-1]
		if parent.Left == nil {
			parent.Left = node
		} else {
			parent.Right = node
		}
		stack = append(stack, node)
	}
	return root
}
