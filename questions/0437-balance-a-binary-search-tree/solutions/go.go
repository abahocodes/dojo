package main

func balanceBst(root *TreeNode) *TreeNode {
	var values []int
	var stack []*TreeNode
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		values = append(values, node.Val)
		node = node.Right
	}
	var build func(lo, hi int) *TreeNode
	build = func(lo, hi int) *TreeNode {
		if lo > hi {
			return nil
		}
		mid := (lo + hi) / 2
		return &TreeNode{Val: values[mid], Left: build(lo, mid-1), Right: build(mid+1, hi)}
	}
	return build(0, len(values)-1)
}
