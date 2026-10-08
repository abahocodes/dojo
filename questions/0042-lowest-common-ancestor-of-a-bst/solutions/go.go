package main

func lowestCommonAncestor(root *TreeNode, p int, q int) int {
	lo, hi := min(p, q), max(p, q)
	node := root
	for {
		if hi < node.Val {
			node = node.Left
		} else if lo > node.Val {
			node = node.Right
		} else {
			return node.Val
		}
	}
}
