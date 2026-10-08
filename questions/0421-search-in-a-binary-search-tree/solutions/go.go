package main

func searchBst(root *TreeNode, val int) *TreeNode {
	node := root
	for node != nil && node.Val != val {
		if val < node.Val {
			node = node.Left
		} else {
			node = node.Right
		}
	}
	return node
}
