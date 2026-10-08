package main

func mergeTrees(root1 *TreeNode, root2 *TreeNode) *TreeNode {
	if root1 == nil {
		return root2
	}
	stack := [][2]*TreeNode{{root1, root2}}
	for len(stack) > 0 {
		pair := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		a, b := pair[0], pair[1]
		if b == nil {
			continue
		}
		a.Val += b.Val
		if a.Left == nil {
			a.Left = b.Left
		} else {
			stack = append(stack, [2]*TreeNode{a.Left, b.Left})
		}
		if a.Right == nil {
			a.Right = b.Right
		} else {
			stack = append(stack, [2]*TreeNode{a.Right, b.Right})
		}
	}
	return root1
}
