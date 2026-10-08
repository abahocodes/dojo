package main

func isSameTree(p *TreeNode, q *TreeNode) bool {
	stack := [][2]*TreeNode{{p, q}}
	for len(stack) > 0 {
		pair := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		a, b := pair[0], pair[1]
		if a == nil && b == nil {
			continue
		}
		if a == nil || b == nil || a.Val != b.Val {
			return false
		}
		stack = append(stack, [2]*TreeNode{a.Left, b.Left}, [2]*TreeNode{a.Right, b.Right})
	}
	return true
}
