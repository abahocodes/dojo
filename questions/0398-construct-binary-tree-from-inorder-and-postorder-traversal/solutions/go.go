package main

func buildTreeInPost(inorder []int, postorder []int) *TreeNode {
	n := len(postorder)
	if n == 0 {
		return nil
	}
	// Walk postorder backwards (root, right, left) and inorder backwards.
	root := &TreeNode{Val: postorder[n-1]}
	stack := []*TreeNode{root}
	i := n - 1
	for j := n - 2; j >= 0; j-- {
		node := &TreeNode{Val: postorder[j]}
		parent := stack[len(stack)-1]
		if parent.Val != inorder[i] {
			parent.Right = node
		} else {
			for len(stack) > 0 && stack[len(stack)-1].Val == inorder[i] {
				parent = stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				i--
			}
			parent.Left = node
		}
		stack = append(stack, node)
	}
	return root
}
