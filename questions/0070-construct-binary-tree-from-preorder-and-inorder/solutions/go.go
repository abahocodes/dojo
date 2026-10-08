package main

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}
	root := &TreeNode{Val: preorder[0]}
	stack := []*TreeNode{root}
	j := 0 // next inorder position not yet closed off
	for _, val := range preorder[1:] {
		node := stack[len(stack)-1]
		if node.Val != inorder[j] {
			// node's left subtree is not finished, so val is its left child
			node.Left = &TreeNode{Val: val}
			stack = append(stack, node.Left)
		} else {
			// pop every node whose left side is complete; the last one popped
			// is the node whose right child val is
			for len(stack) > 0 && stack[len(stack)-1].Val == inorder[j] {
				node = stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				j++
			}
			node.Right = &TreeNode{Val: val}
			stack = append(stack, node.Right)
		}
	}
	return root
}
