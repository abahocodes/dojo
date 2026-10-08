package main

func isSymmetric(root *TreeNode) bool {
	type pair struct{ a, b *TreeNode }
	stack := []pair{{root.Left, root.Right}}
	for len(stack) > 0 {
		p := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if p.a == nil && p.b == nil {
			continue
		}
		if p.a == nil || p.b == nil || p.a.Val != p.b.Val {
			return false
		}
		stack = append(stack, pair{p.a.Left, p.b.Right}, pair{p.a.Right, p.b.Left})
	}
	return true
}
