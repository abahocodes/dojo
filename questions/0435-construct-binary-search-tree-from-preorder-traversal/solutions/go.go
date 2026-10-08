package main

func bstFromPreorder(preorder []int) *TreeNode {
	root := &TreeNode{Val: preorder[0]}
	stack := []*TreeNode{root}
	for _, v := range preorder[1:] {
		node := &TreeNode{Val: v}
		if v < stack[len(stack)-1].Val {
			stack[len(stack)-1].Left = node
		} else {
			parent := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			for len(stack) > 0 && stack[len(stack)-1].Val < v {
				parent = stack[len(stack)-1]
				stack = stack[:len(stack)-1]
			}
			parent.Right = node
		}
		stack = append(stack, node)
	}
	return root
}
