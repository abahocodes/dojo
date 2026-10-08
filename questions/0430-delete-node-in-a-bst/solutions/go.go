package main

func deleteNode(root *TreeNode, key int) *TreeNode {
	var parent *TreeNode
	node := root
	for node != nil && node.Val != key {
		parent = node
		if key < node.Val {
			node = node.Left
		} else {
			node = node.Right
		}
	}
	if node == nil {
		return root
	}

	if node.Left != nil && node.Right != nil {
		succParent, succ := node, node.Right
		for succ.Left != nil {
			succParent, succ = succ, succ.Left
		}
		node.Val = succ.Val
		if succParent == node {
			succParent.Right = succ.Right
		} else {
			succParent.Left = succ.Right
		}
		return root
	}

	child := node.Left
	if child == nil {
		child = node.Right
	}
	if parent == nil {
		return child
	}
	if parent.Left == node {
		parent.Left = child
	} else {
		parent.Right = child
	}
	return root
}
