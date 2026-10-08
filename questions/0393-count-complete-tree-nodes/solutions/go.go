package main

func countNodes(root *TreeNode) int {
	leftDepth := func(node *TreeNode) int {
		depth := 0
		for node != nil {
			depth++
			node = node.Left
		}
		return depth
	}
	count := 0
	node := root
	for node != nil {
		lh := leftDepth(node.Left)
		rh := leftDepth(node.Right)
		if lh == rh {
			count += 1 << lh
			node = node.Right
		} else {
			count += 1 << rh
			node = node.Left
		}
	}
	return count
}
