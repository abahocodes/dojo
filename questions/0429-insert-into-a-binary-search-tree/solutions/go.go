package main

func insertIntoBst(root *TreeNode, val int) *TreeNode {
	node := &TreeNode{Val: val}
	if root == nil {
		return node
	}
	cur := root
	for {
		if val < cur.Val {
			if cur.Left == nil {
				cur.Left = node
				return root
			}
			cur = cur.Left
		} else {
			if cur.Right == nil {
				cur.Right = node
				return root
			}
			cur = cur.Right
		}
	}
}
